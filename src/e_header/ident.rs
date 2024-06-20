#[derive(Debug)]
pub enum Class {
    ELF32,
    ELF64,
}

#[derive(Debug)]
pub enum Endian {
    BigEndian,
    LittleEndian,
}

#[derive(Debug)]
pub enum Version {
    Version1,
}

pub struct Ident {
    ident: [u8; 16],
}

#[derive(Debug)]
pub enum OS {
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    GNUHurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    NovellModesto,
    OpenBSD,
    OpenVMS,
}

impl From<u8> for OS {
    fn from(value: u8) -> Self {
        match value {
            0x00 => OS::SystemV,
            0x01 => OS::HPUX,
            0x02 => OS::NetBSD,
            0x03 => OS::Linux,
            0x04 => OS::GNUHurd,
            0x06 => OS::Solaris,
            0x07 => OS::AIX,
            0x08 => OS::IRIX,
            0x09 => OS::FreeBSD,
            0x0a => OS::Tru64,
            0x0b => OS::NovellModesto,
            0x0c => OS::OpenBSD,
            0x0d => OS::OpenVMS,
            _ => panic!("unrecognized OS {}", value),
        }
    }
}

impl Ident {
    pub fn new(data: [u8; 16]) -> Self {
        Self { ident: data }
    }
    pub fn check(&self) {
        let expect = [0x7f, 0x45, 0x4c, 0x46];
        for (i, c) in expect.iter().enumerate() {
            assert!(self.ident[i] == *c, "wrong magic");
        }
    }
    pub fn class(&self) -> Class {
        match self.ident[4] {
            1 => Class::ELF32,
            2 => Class::ELF64,
            _ => {
                panic!("wrong class type {}", self.ident[4])
            }
        }
    }

    pub fn endian(&self) -> Endian {
        match self.ident[5] {
            1 => Endian::LittleEndian,
            2 => Endian::BigEndian,
            _ => {
                panic!("wrong endian type {}", self.ident[5])
            }
        }
    }
    pub fn version(&self) -> Version {
        assert!(self.ident[6] == 1);
        Version::Version1
    }
    pub fn os(&self) -> OS {
        self.ident[7].into()
    }
}
