#[derive(Debug)]
pub enum ISA {
    X86,
    Intel80860,
    IA64,
    AMDx86_64,
    AArch64,
    RiscV,
}

impl From<u16> for ISA {
    fn from(value: u16) -> Self {
        match value {
            0xf3 => ISA::RiscV,
            0xb7 => ISA::AArch64,
            0x3e => ISA::AMDx86_64,
            0x32 => ISA::IA64,
            0x07 => ISA::Intel80860,
            0x03 => ISA::X86,
            _ => panic!("unsupported ISA {}", value),
        }
    }
}
