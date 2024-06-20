#[derive(Debug)]
pub enum EType {
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
}

impl From<u16> for EType {
    fn from(value: u16) -> Self {
        match value {
            0x00 => EType::NONE,
            0x01 => EType::REL,
            0x02 => EType::EXEC,
            0x03 => EType::DYN,
            0x04 => EType::CORE,
            _ => panic!("unsupported elf type {}", value),
        }
    }
}
