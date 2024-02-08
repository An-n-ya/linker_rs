use std::{fs::File, io::{Cursor, Read}};

use crate::linker::{ElfHeader, SectionHeader};

use super::read_struct::read_struct;

pub struct InputFile {
    #[allow(dead_code)]
    file: File,
    pub elf_sections: Vec<SectionHeader>
}

impl InputFile {
    pub fn new(mut file: File) -> Self {
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        let mut cursor = Cursor::new(contents);
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

        Self {
            file,
            elf_sections: sections
        }
    }
}