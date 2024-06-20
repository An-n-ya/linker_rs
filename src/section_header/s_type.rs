use core::panic;

#[derive(Debug)]
pub enum SType {
    NULL,
    PROG,
    SYMTAB,
    STRTAB,
    RELA,
    HASH,
    DYNAMIC,
    NOTE,
    NOBITS,
    REL,
    SHLIB,
    DNYSYM,
}

impl From<u32> for SType {
    fn from(value: u32) -> Self {
        match value {
            0x00 => SType::NULL,
            0x01 => SType::PROG,
            0x02 => SType::SYMTAB,
            0x03 => SType::STRTAB,
            0x04 => SType::RELA,
            0x05 => SType::HASH,
            0x06 => SType::DYNAMIC,
            0x07 => SType::NOTE,
            0x08 => SType::NOBITS,
            0x09 => SType::REL,
            0x0a => SType::SHLIB,
            0x0b => SType::DNYSYM,
            _ => panic!("unrecognized section type {}", value),
        }
    }
}
