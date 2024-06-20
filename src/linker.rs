use core::fmt;

use crate::e_header::{self, ident::Ident};

#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader {
    pub ident: [u8; 16],
    pub _type: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u64,
    pub ph_off: u64,
    pub sh_off: u64,
    pub flags: u32,
    pub eh_size: u16,
    pub ph_ent_size: u16,
    pub ph_num: u16,
    pub sh_ent_size: u16,
    pub sh_num: u16,
    pub sh_strndx: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct SectionHeader {
    pub name: u32,
    pub _type: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub add_align: u64,
    pub ent_size: u64,
}

impl ElfHeader {
    pub fn parse_ident(&self) -> Ident {
        Ident::new(self.ident.clone())
    }
}

impl fmt::Display for ElfHeader {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ident = self.parse_ident();
        println!("Class:\t\t{:?}", ident.class());
        println!("Endianness:\t{:?}", ident.endian());
        println!("Version:\t{:?}", ident.version());
        println!("OS:\t\t{:?}", ident.os());
        println!("type:\t\t{:?}", e_header::e_type::EType::from(self._type));
        println!("ISA:\t\t{:?}", e_header::isa::ISA::from(self.machine));
        println!("entry:\t\t{:#X}", self.entry);

        Ok(())
    }
}
