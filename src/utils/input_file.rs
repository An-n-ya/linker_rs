use core::fmt;
use std::{
    fs::File,
    io::{Cursor, Read, Write},
};

use crate::linker::{ElfHeader, ElfSymbol, SectionHeader, SectionType};

use super::{read_struct::read_struct, str_table::StrTable};

pub struct ElfData {
    pub name: String,
    pub elf_header: ElfHeader,
    pub elf_sections: Vec<SectionHeader>,
    pub symbol_info: Option<SymbolInfo>,
    pub is_alive: bool,
    sec_str_tab: StrTable,
}

pub struct SymbolInfo {
    pub elf_symbols: Vec<ElfSymbol>,
    pub first_global: usize,
    pub str_tab: StrTable,
}

impl ElfData {
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

        Self {
            name,
            elf_header,
            is_alive: false,
            symbol_info,
            elf_sections: sections,
            sec_str_tab: table,
        }
    }
    pub fn find_section(&self, typ: SectionType) -> Option<SectionHeader> {
        for s in &self.elf_sections {
            if s._type == typ {
                return Some(s.clone());
            }
        }
        None
    }
}

impl fmt::Display for ElfData {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("ELF Headers:");
        println!("{}", self.elf_header);

        println!("str table: {:?}", self.sec_str_tab);

        println!("\nSection Headers:");
        println!("[Nr] Name\t\tType\t\tAddr\t\tOffset\t\tSize\t\tES\tFlg\tLk\tInf\tAl");
        for (i, sec) in self.elf_sections.iter().enumerate() {
            let mut name = self.sec_str_tab.get(sec.name as usize);
            name.truncate(10);
            if name.len() == 10 {
                name.push_str("..");
            }
            println!(
                "[{i:02}] {:<12}\t{:?}\t{:08x}\t{:08x}\t{:08x}\t{}\t{}\t{}\t{}\t{}",
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
