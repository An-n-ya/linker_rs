use std::{collections::HashMap, rc::Rc, sync::Mutex};

use crate::{context::Context, Id};

use super::output_section::{OutputSection, SectionWrapper, ShareOutputSection};

pub struct MergedSection {
    section: SectionWrapper,
    map: HashMap<String, SectionFragment>,
}

pub struct SectionFragment {
    section_id: Id,
}

impl OutputSection for MergedSection {
    fn is_mergable(&self) -> bool {
        true
    }

    fn section_header(&self) -> &crate::linker::SectionHeader {
        &self.section.elf_header
    }

    fn name(&self) -> String {
        self.section.name.clone()
    }
}

impl MergedSection {
    pub fn new(section: SectionWrapper) -> ShareOutputSection {
        let sec = MergedSection {
            section,
            map: HashMap::default(),
        };
        let sec = Rc::new(Mutex::new(sec));
        sec
    }
}
