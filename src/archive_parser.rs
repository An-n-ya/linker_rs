use std::{
    fs::File,
    io::{Cursor, Read},
    path::PathBuf,
    str::from_utf8_unchecked,
};

use crate::utils::{input_file::ElfData, read_struct::read_struct, str_table::StrTable};

pub struct Parser {
    libraries: Option<Vec<PathBuf>>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ArchiveEntry {
    file_name: [u8; 16],
    timestamp: [u8; 12],
    owner_id: [u8; 6],
    group_id: [u8; 6],
    file_mode: [u8; 8],
    file_size: [u8; 10],
    end_marker: u16,
}

impl ArchiveEntry {
    pub fn get_size(&self) -> usize {
        let size_str = unsafe { from_utf8_unchecked(&self.file_size) }.to_string();
        let size = usize::from_str_radix(size_str.trim(), 10)
            .expect(&format!("invalid size str {}", size_str));
        size
    }

    pub fn get_name(&self, string_table: &Option<StrTable>) -> String {
        if !self.is_string_table() && !self.is_symbol_table() && self.file_name[0] == '/' as u8 {
            // long name
            if let Some(ref table) = string_table {
                let offset_str = unsafe { from_utf8_unchecked(&self.file_name[1..]) }.to_string();
                let offset = usize::from_str_radix(offset_str.trim(), 10)
                    .expect(&format!("invalid offset size {}", offset_str));
                table.get(offset).trim().to_string()
            } else {
                panic!("cannot find string table");
            }
        } else {
            let name = unsafe { from_utf8_unchecked(&self.file_name) }.to_string();
            name.trim().to_string()
        }
    }

    fn has_prefix(&self, s: &str) -> bool {
        for (i, c) in s.chars().enumerate() {
            if self.file_name[i] != c as u8 {
                return false;
            }
        }
        true
    }

    pub fn is_symbol_table(&self) -> bool {
        self.has_prefix("/ ")
    }

    pub fn is_string_table(&self) -> bool {
        self.has_prefix("// ")
    }
}

impl Parser {
    const SIGNATURE: &str = "!<arch>\n";
    pub fn new(libraries: Option<Vec<PathBuf>>) -> Self {
        Self { libraries }
    }

    pub fn parse(&self, archive: String) -> Vec<ElfData> {
        if let Some(libraries) = &self.libraries {
            for path in libraries {
                let archive = format!("lib{}.a", archive);
                let mut path = path.clone();
                path.push(archive);
                if let Ok(file) = File::open(&path) {
                    dbg!("find archive", path);
                    return Self::parse_inner(file);
                }
            }
            panic!("cannot find archive {archive}");
        } else {
            panic!("don't have any libraries to parse this archive: {archive}");
        }
    }

    fn parse_inner(mut file: File) -> Vec<ElfData> {
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        let total = contents.len() as u64;
        let mut cursor = Cursor::new(contents);
        let mut string_table = None;
        let mut elf_data = vec![];
        let mut sig = [0; 8];
        cursor.read_exact(&mut sig).expect("cannot read signature");
        for (i, c) in Self::SIGNATURE.chars().enumerate() {
            if sig[i] as char != c {
                panic!("signature doesn't match, got {:?}", sig);
            }
        }
        while cursor.position() < total {
            let archive_entry = read_struct::<ArchiveEntry, Cursor<_>>(&mut cursor)
                .ok()
                .unwrap();
            let mut pos = cursor.position();
            // align 2
            if pos % 2 != 0 {
                pos += 1;
            }
            cursor.set_position(pos);
            let mut data = vec![0; archive_entry.get_size()];
            cursor.read_exact(&mut data).expect(&format!(
                "cannot read data of entry {}",
                archive_entry.get_name(&string_table)
            ));
            if archive_entry.is_symbol_table() {
                // skip this one
                continue;
            } else if archive_entry.is_string_table() {
                let size = data.len();
                string_table = Some(StrTable::new(data, size));
            } else {
                let elf: ElfData =
                    ElfData::new_from_buf(&data, archive_entry.get_name(&string_table));
                elf_data.push(elf);
            }
        }
        elf_data
    }
}
