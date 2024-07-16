use core::fmt;
use std::{
    fs::File,
    io::{Cursor, Read, Write},
};

use crate::{
    linker::{ElfHeader, SectionHeader},
    section_header,
};

use super::{read_struct::read_struct, str_table::StrTable};

pub struct ElfData {
    pub name: String,
    pub elf_header: ElfHeader,
    pub elf_sections: Vec<SectionHeader>,
    sec_str_tab: StrTable,
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
        for _ in 1..section_num {
            sections.push(read_struct(&mut cursor).ok().unwrap())
        }

        // section string table
        let idx = elf_header.sh_strndx;
        let sec = &sections[idx as usize];
        let offset = sec.offset;
        let size = sec.size;
        let mut buf = vec![0u8; size as usize];
        cursor.set_position(offset);
        cursor
            .read_exact(buf.by_ref())
            .expect("fail to read str table of headers");
        let table = StrTable::new(buf, size as usize);

        Self {
            name,
            elf_header,
            elf_sections: sections,
            sec_str_tab: table,
        }
    }
}

impl fmt::Display for ElfData {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("ELF Headers:");
        println!("{}", self.elf_header);

        println!("str table: {:?}", self.sec_str_tab);

        println!("\nSection Headers:");
        println!("[Nr] Name\t\tType\tAddr\t\tOffset\t\tSize\t\tES\tFlg\tLk\tInf\tAl");
        for (i, sec) in self.elf_sections.iter().enumerate() {
            let mut name = self.sec_str_tab.get(sec.name as usize);
            name.truncate(10);
            if name.len() == 10 {
                name.push_str("..");
            }
            println!(
                "[{i:02}] {:<12}\t{:?}\t{:08x}\t{:08x}\t{:08x}\t{}\t{}\t{}\t{}\t{}",
                name,
                section_header::s_type::SType::from(sec._type),
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
        Ok(())
    }
}
