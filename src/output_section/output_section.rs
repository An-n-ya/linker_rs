use std::{rc::Rc, sync::Mutex};

use crate::{
    context::Context,
    linker::{SectionHeader, SectionType},
    Id,
};

pub type ShareOutputSection = Rc<Mutex<dyn OutputSection>>;

pub trait OutputSection {
    fn is_mergable(&self) -> bool {
        false
    }
    fn section_header(&self) -> &SectionHeader;

    fn name(&self) -> String;
    fn typ(&self) -> SectionType {
        self.section_header()._type
    }
    fn flags(&self) -> u64 {
        self.section_header().flags
    }
}

pub struct SectionWrapper {
    pub name: String,
    pub id: Id,
    pub elf_header: SectionHeader,
}

impl SectionWrapper {
    pub fn new(id: usize) -> Self {
        let id = Rc::new(Mutex::new(id));
        Self {
            id,
            name: "".to_string(),
            elf_header: SectionHeader::default(),
        }
    }
}
