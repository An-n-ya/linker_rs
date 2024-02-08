
#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader {
    pub ident: [u8;16],
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
