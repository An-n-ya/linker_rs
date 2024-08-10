use core::fmt;
use std::{
    fs::File,
    io::{Cursor, Read, Write},
    rc::Rc,
    str::from_utf8,
    sync::Mutex,
};

use crate::{
    context::Context,
    linker::{ElfHeader, ElfSymbol, SectionFlag, SectionHeader, SectionIndex, SectionType},
    output_section::{
        merged_section::{FragmentData, ShareSectionFragment},
        output_section::ShareOutputSection,
    },
    section::Section,
    symbol::{self, ShareSymbol, Symbol},
};

use super::{read_struct::read_struct, str_table::StrTable};

pub struct InputElf {
    pub name: String,
    pub elf_header: ElfHeader,
    pub section_info: SectionInfo,
    pub symbol_info: Option<SymbolInfo>,
    pub is_alive: bool,
    pub id: usize,
}

pub struct SectionInfo {
    pub elf_sections: Vec<SectionHeader>,
    pub str_tab: StrTable,
    pub sections: Vec<Option<Section>>,
    pub mergeable_sections: Vec<Option<InputMergeableSection>>,
}

struct InputMergeableSection {
    pub parent: ShareOutputSection,
    pub fragments: Vec<ShareSectionFragment>,
    pub data: Vec<FragmentData>,
    pub offset: Vec<usize>,
}
impl InputMergeableSection {
    pub fn new(parent: ShareOutputSection) -> Self {
        Self {
            parent,
            fragments: vec![],
            data: vec![],
            offset: vec![],
        }
    }
    pub fn get_fragment(&self, offset: usize) -> ShareSectionFragment {
        let mut ind = 0;
        for (i, o) in self.offset.iter().enumerate() {
            if *o >= offset {
                ind = i;
                break;
            }
        }
        self.fragments[ind].clone()
    }
}

pub struct SymbolInfo {
    pub elf_symbols: Vec<ElfSymbol>,
    pub first_global: usize,
    pub str_tab: StrTable,
    pub local_symbols: Vec<Symbol>,
    pub global_symbols: Vec<ShareSymbol>,
}

impl SymbolInfo {}

impl InputElf {
    pub fn new(mut file: File, name: String) -> Self {
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        Self::new_from_buf(&contents, name)
    }

    pub fn new_from_buf(data: &[u8], name: String) -> Self {
        let mut cursor = Cursor::new(data);
        let elf_header: ElfHeader = read_struct(&mut cursor).ok().unwrap();
        let section_offset = elf_header.sh_off;
        let mut section_num = elf_header.sh_num as u64;

        // cursor.seek(SeekFrom::Current(section_offset as i64)).unwrap();
        cursor.set_position(section_offset);
        let section_header: SectionHeader = read_struct(&mut cursor).ok().unwrap();

        if section_num == 0 {
            section_num = section_header.size;
        }

        let mut sections = vec![section_header];
        let mut symbol_table_section = None;
        for _ in 1..section_num {
            let sec: SectionHeader = read_struct(&mut cursor).ok().unwrap();
            if sec._type == SectionType::SYMTAB {
                symbol_table_section = Some(sec.clone());
            }
            sections.push(sec);
        }

        fn read_section_data(
            cursor: &mut std::io::Cursor<&[u8]>,
            section: &SectionHeader,
        ) -> Vec<u8> {
            let (offset, size) = (section.offset, section.size);
            let mut buf = vec![0u8; size as usize];
            cursor.set_position(offset);
            cursor.read_exact(buf.by_ref()).ok();
            buf
        }

        // section string table
        let buf = read_section_data(&mut cursor, &sections[elf_header.sh_strndx as usize]);
        let size = buf.len();
        let table = StrTable::new(buf, size);
        // parse symbol
        let mut symbol_info = None;
        if let Some(table) = symbol_table_section {
            let str_table_idx = table.link as usize;
            let buf = read_section_data(&mut cursor, &sections[str_table_idx]);
            let size = buf.len();
            let str_table = StrTable::new(buf, size);
            let mut info = SymbolInfo {
                elf_symbols: vec![],
                first_global: table.info as usize,
                str_tab: str_table,
                local_symbols: vec![],
                global_symbols: vec![],
            };
            let (offset, size) = (table.offset, table.size);
            cursor.set_position(offset);
            let mut cur = 0;
            while cur < size {
                let symbol: ElfSymbol = read_struct(&mut cursor).ok().unwrap();
                cur += size_of::<ElfSymbol>() as u64;
                info.elf_symbols.push(symbol);
            }
            symbol_info = Some(info);
        }
        let mut section_info = SectionInfo {
            elf_sections: sections,
            sections: vec![],
            mergeable_sections: vec![],
            str_tab: table,
        };
        for (i, sec) in section_info.elf_sections.iter().enumerate() {
            use SectionType::*;
            match sec._type {
                SYMTAB | REL | RELA | STRTAB | NULL => {
                    section_info.sections.push(None);
                }
                _ => {
                    let name = section_info.str_tab.get(sec.name as usize);
                    let data = read_section_data(&mut cursor, sec);
                    let section = Section {
                        elf: 0, // this is a temporary id
                        name,
                        index: i,
                        data,
                    };

                    section_info.sections.push(Some(section));
                }
            }
        }

        Self {
            name,
            elf_header,
            is_alive: false,
            symbol_info,
            section_info,
            id: 0,
        }
    }
    pub fn find_section(&self, typ: SectionType) -> Option<SectionHeader> {
        for s in &self.section_info.elf_sections {
            if s._type == typ {
                return Some(s.clone());
            }
        }
        None
    }

    fn initialize_mergeable_section(&mut self, ctx: &mut Context) {
        let total = self.section_info.elf_sections.len();
        for i in 0..total {
            let elf_sec = &self.section_info.elf_sections[i];
            if (elf_sec.flags & SectionFlag::MERGE as u64) != 0 {
                if let Some(ref sec) = &self.section_info.sections[i] {
                    let name = sec.name.clone();
                    let typ = elf_sec._type;
                    let flags = elf_sec.flags;
                    let out_sec = ctx.find_mergeable_section(name, typ, flags);
                    let mut mergeable_section = InputMergeableSection::new(out_sec.clone());
                    let out_sec_guard = out_sec.lock().unwrap();
                    assert!(out_sec_guard.is_mergeable());

                    if (elf_sec.flags & SectionFlag::STRINGS as u64) != 0 {
                        let mut s_data: Vec<u8> = vec![];
                        // let mut strings = vec![];
                        let mut offset = 0;
                        let size = elf_sec.ent_size as usize;
                        for chunk in sec.data.chunks(size) {
                            if chunk.iter().all(|&x| x == 0) {
                                // FIXME: need to handle empty string
                                let s = {
                                    // FIXME: how to handle utf8
                                    if let Ok(s) = from_utf8(&s_data) {
                                        s.to_string()
                                    } else {
                                        let s: String = s_data.iter().map(|&c| c as char).collect();
                                        s
                                    }
                                };
                                // strings.push(s.clone());
                                s_data.clear();
                                mergeable_section.data.push(FragmentData::Str(s));
                                mergeable_section.offset.push(offset);
                            } else {
                                s_data.extend(chunk);
                            }
                            offset += size;
                        }
                    } else {
                        // constants
                        let mut offset = 0;
                        let size = elf_sec.ent_size as usize;
                        for chunk in sec.data.chunks(size) {
                            mergeable_section
                                .data
                                .push(FragmentData::Constant(Vec::from(chunk)));
                            mergeable_section.offset.push(offset);
                            offset += size;
                        }
                    }
                    self.section_info
                        .mergeable_sections
                        .push(Some(mergeable_section));
                } else {
                    panic!("mergeable section doesn't exist");
                }
            } else {
                self.section_info.mergeable_sections.push(None);
            }
        }

        // iterate over all mergeable section
        // - for each elf object, pollute ctx's merged section
        // - for each elf object, pollute each sym's section fragment (update offset etc.)
    }

    pub fn initialize_section(&mut self, ctx: &mut Context) {
        for sec in self.section_info.sections.iter_mut() {
            if let Some(sec) = sec {
                sec.elf = self.id;
            }
        }
        self.initialize_mergeable_section(ctx);

        // register symbol to output mergeable section
        self.register_mergeable_section();
    }

    fn register_mergeable_section(&mut self) {
        let n = self.section_info.elf_sections.len();
        for i in 0..n {
            let elf = &self.section_info.elf_sections[i];
            let sec = &mut self.section_info.mergeable_sections[i];
            if let Some(sec) = sec {
                let mut parent = sec.parent.lock().unwrap();
                assert!(parent.is_mergeable());
                if let Some(merge) = parent.to_mergeable() {
                    for frag in &sec.data {
                        // FIXME: is it right to use the ent_size?
                        let sec_frag = merge.insert(frag, elf.ent_size as usize);
                        sec.fragments.push(sec_frag);
                    }
                }
            }
        }

        // iterate each sym
        // TODO: write a iterator over every symbol
        self.for_each_sym(|sym, elf, sec_info| {
            let sec_idx = {
                match elf.index() {
                    SectionIndex::ABS | SectionIndex::COMMON | SectionIndex::UNDEF => {
                        return;
                    }
                    SectionIndex::Other(a) => a,
                    _ => {
                        panic!("unsupported Section Index");
                    }
                }
            };
            if let Some(ref sec) = sec_info.mergeable_sections[sec_idx as usize] {
                ///////////////////// DEBUG //////////////////////////
                // let sec_header = &sec_info.sections[sec_idx as usize];
                // if let Some(ref sec_header) = sec_header {
                //     println!(
                //         "handling sec {}, elf.val {}, sec_data {:?}",
                //         sec_header.name, elf.val, sec_header.data
                //     );
                // }
                ///////////////////// DEBUG //////////////////////////
                let frag = sec.get_fragment(elf.val as usize);
                sym.set_frag(frag);
            }
        });
    }

    fn for_each_sym<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Symbol, &ElfSymbol, &SectionInfo),
    {
        if let Some(symbol_info) = &mut self.symbol_info {
            for i in 0..symbol_info.first_global {
                let elf = &symbol_info.elf_symbols[i];
                let sym = &mut symbol_info.local_symbols[i];
                f(sym, elf, &self.section_info);
            }
            for i in symbol_info.first_global..symbol_info.elf_symbols.len() {
                let ind = i - symbol_info.first_global;
                let elf = &symbol_info.elf_symbols[ind];
                let sym = symbol_info.global_symbols[ind].clone();
                let mut sym_guard = sym.lock().unwrap();
                f(&mut sym_guard, elf, &self.section_info);
            }
        }
    }

    pub fn initialize_symbol(&mut self, ctx: &mut Context) {
        if let Some(ref mut info) = &mut self.symbol_info {
            let global_index = info.first_global;
            for i in 0..global_index {
                let elf_sym = &info.elf_symbols[i];
                let name = elf_sym.name(&info.str_tab);
                let value = elf_sym.val as usize;
                info.local_symbols.push(Symbol::new(name, i, value))
            }
            for i in global_index..info.elf_symbols.len() {
                let elf_sym = &info.elf_symbols[i];
                let name = elf_sym.name(&info.str_tab);
                info.global_symbols.push(ctx.find_symbol_by_name(name));
            }
        }
    }

    pub fn resolve_symbol(&mut self) {
        if let Some(ref mut info) = &mut self.symbol_info {
            let start = info.first_global;
            for i in 0..info.global_symbols.len() {
                let sym = info.global_symbols[i].clone();
                let mut sym = sym.lock().unwrap();
                let elf_sym = &info.elf_symbols[start + i];
                // assert!(
                //     elf_sym.bind() == SymbolBinding::GLOBAL,
                //     "expect global symbol got {:?}",
                //     elf_sym.bind()
                // );
                match elf_sym.index() {
                    SectionIndex::Other(_) => {
                        if sym.elf.is_none() {
                            sym.elf = Some(self.id);
                            sym.index = i + start;
                            sym.value = elf_sym.val as usize;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn clear_symbol(&self) {
        if let Some(ref info) = &self.symbol_info {
            for i in 0..info.global_symbols.len() {
                let sym = info.global_symbols[i].clone();
                let mut sym = sym.lock().unwrap();
                // let elf_sym = &info.elf_symbols[start + i];
                // assert!(
                //     elf_sym.bind() == SymbolBinding::GLOBAL,
                //     "expect global symbol got {:?}",
                //     elf_sym.bind()
                // );

                if let Some(id) = sym.elf {
                    if id == self.id {
                        sym.is_alive = false;
                    }
                }
            }
        }
    }

    pub fn mark_live_objects<F>(&self, ctx: &Context, mut f: F)
    where
        F: FnMut(Rc<Mutex<InputElf>>),
    {
        assert!(self.is_alive);

        if let Some(ref info) = &self.symbol_info {
            let start = info.first_global;
            for i in 0..info.global_symbols.len() {
                let sym = info.global_symbols[i].clone();
                let sym = sym.lock().unwrap();
                let elf_sym = &info.elf_symbols[start + i];

                if let Some(id) = sym.elf {
                    if id == self.id {
                        continue;
                    }
                    let elf = ctx
                        .get_object(id)
                        .expect(&format!("cannot find elf, id: {}", id));
                    let mut elf_guard = elf.lock().unwrap();

                    if elf_sym.index() == SectionIndex::UNDEF && !elf_guard.is_alive {
                        elf_guard.is_alive = true;
                        f(elf.clone());
                    }
                }
            }
        }
    }
}

impl fmt::Display for InputElf {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("ELF Headers:");
        println!("{}", self.elf_header);

        println!("str table: {:?}", self.section_info.str_tab);

        println!("\nSection Headers:");
        println!("[Nr] Name\t\tType\t\tAddr\t\tOffset\t\tSize\t\tES\tFlg\tLk\tInf\tAl");
        for (i, sec) in self.section_info.elf_sections.iter().enumerate() {
            let mut name = self.section_info.str_tab.get(sec.name as usize);
            name.truncate(10);
            if name.len() == 10 {
                name.push_str("..");
            }
            println!(
                "[{i:02}] {:<12}\t{:?}\t{:08x}\t{:08x}\t{:08x}\t{}\t{:?}\t{}\t{}\t{}",
                name,
                sec._type,
                sec.addr,
                sec.offset,
                sec.size,
                sec.ent_size,
                sec.flags,
                sec.link,
                sec.info,
                sec.add_align
            );
        }
        if let Some(table) = &self.symbol_info {
            println!();
            println!("{}", table);
        }
        Ok(())
    }
}

impl fmt::Display for SymbolInfo {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("[Num]: value\tSize\tType\tBind\tIndex\tName");
        for (i, symbol) in self.elf_symbols.iter().enumerate() {
            println!(
                "{}:\t{}\t{}\t{:?}\t{:?}\t{:?}\t{}",
                i,
                symbol.val,
                symbol.size,
                symbol.typ(),
                symbol.bind(),
                symbol.index(),
                symbol.name(&self.str_tab)
            )
        }
        Ok(())
    }
}
