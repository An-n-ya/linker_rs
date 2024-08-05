use core::fmt;

use crate::{
    e_header::{self, ident::Ident},
    utils::str_table::StrTable,
};

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
#[derive(Debug, Clone, Default)]
pub struct SectionHeader {
    pub name: u32,
    pub _type: SectionType,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub add_align: u64,
    pub ent_size: u64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ElfSymbol {
    pub name: u32,
    pub info: u8,  // This member specifies the symbol's type and binding attributes.
    pub other: u8, // This member currently holds 0 and has no defined meaning.
    pub shndx: u16,
    pub val: u64,
    pub size: u64,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types, unused)]
pub enum SectionType {
    DYNAMIC = 0x6,
    DYNSYM = 0xb,
    FINI_ARRAY = 0xf,
    HASH = 0x5,
    HIPROC = 0x7fffffff,
    HIUSER = 0xffffffff,
    INIT_ARRAY = 0xe,
    LOPROC = 0x70000000,
    LOUSER = 0x80000000,
    NOBITS = 0x8,
    NOTE = 0x7,
    NULL = 0x0,
    PREINIT_ARRAY = 0x10,
    PROGBITS = 0x1,
    REL = 0x9,
    RELA = 0x4,
    SHLIB = 0xa,
    STRTAB = 0x3,
    SYMTAB = 0x2,
}
impl Default for SectionType {
    fn default() -> Self {
        Self::NULL
    }
}
#[repr(u64)]
#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types, unused)]
pub enum SectionFlag {
    WRITE = (1 << 0),            /* Writable */
    ALLOC = (1 << 1),            /* Occupies memory during execution */
    EXECINSTR = (1 << 2),        /* Executable */
    MERGE = (1 << 4),            /* Might be merged */
    STRINGS = (1 << 5),          /* Contains nul-terminated strings */
    INFO_LINK = (1 << 6),        /* `sh_info' contains SHT index */
    LINK_ORDER = (1 << 7),       /* Preserve order after combining */
    OS_NONCONFORMING = (1 << 8), /* Non-standard OS specific handling required */
    GROUP = (1 << 9),            /* Section is member of a group.  */
    TLS = (1 << 10),             /* Section hold thread-local data.  */
    COMPRESSED = (1 << 11),      /* Section with compressed data. */
    MASKOS = 0x0ff00000,         /* OS-specific.  */
    MASKPROC = 0xf0000000,       /* Processor-specific */
    ORDERED = (1 << 30),         /* Special ordering requirement (Solaris).  */
    EXCLUDE = (1 << 31),         /* Section is excluded unless referenced or allocated (Solaris).*/
}

#[repr(u16)]
#[derive(PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum SectionIndex {
    UNDEF = 0x0,
    LOPROC = 0xff00,
    HIPROC = 0xff1f,
    ABS = 0xfff1,
    COMMON = 0xfff2,
    HIRESERVE = 0xffff,
    Other(u16) = 1,
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum SymbolBinding {
    LOCAL = 0,
    GLOBAL = 1,
    WEAK = 2,
    LOPROC = 13,
    HIPROC = 15,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum SymbolType {
    NOTYPE = 0,
    OBJECT = 1,
    FUNC = 2,
    SECTION = 3,
    FILE = 4,
    LOPROC = 13,
    HIPROC = 15,
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

impl ElfSymbol {
    pub fn name(&self, str_table: &StrTable) -> String {
        str_table.get(self.name as usize)
    }
    pub fn index(&self) -> SectionIndex {
        SectionIndex::from(self.shndx)
    }
    pub fn typ(&self) -> SymbolType {
        let typ = self.info & 0xf;
        SymbolType::from(typ)
    }
    pub fn bind(&self) -> SymbolBinding {
        let bind = self.info >> 0x4;
        SymbolBinding::from(bind)
    }
    pub fn is_abs(&self) -> bool {
        self.index() == SectionIndex::ABS
    }
    pub fn is_common(&self) -> bool {
        self.index() == SectionIndex::COMMON
    }
}

impl From<u16> for SectionIndex {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::UNDEF,
            0xff00 => Self::LOPROC,
            0xff1f => Self::HIPROC,
            0xfff1 => Self::ABS,
            0xfff2 => Self::COMMON,
            0xffff => Self::HIRESERVE,
            v @ _ => Self::Other(v),
        }
    }
}

impl From<u8> for SymbolType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NOTYPE,
            1 => Self::OBJECT,
            2 => Self::FUNC,
            3 => Self::SECTION,
            4 => Self::FILE,
            13 => Self::LOPROC,
            15 => Self::HIPROC,
            _ => panic!("cannot parse symbol type {}", value),
        }
    }
}
impl From<u8> for SymbolBinding {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::LOCAL,
            1 => Self::GLOBAL,
            2 => Self::WEAK,
            13 => Self::LOPROC,
            15 => Self::HIPROC,
            _ => panic!("cannot parse symbol binding {}", value),
        }
    }
}

impl fmt::Debug for SectionIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UNDEF => write!(f, "UNDEF"),
            Self::LOPROC => write!(f, "LOPROC"),
            Self::HIPROC => write!(f, "HIPROC"),
            Self::ABS => write!(f, "ABS"),
            Self::COMMON => write!(f, "COMMON"),
            Self::HIRESERVE => write!(f, "HIRESERVE"),
            Self::Other(arg0) => write!(f, "{}", arg0),
        }
    }
}
