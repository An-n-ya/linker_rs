use std::{collections::HashMap, rc::Rc, sync::Mutex};

use crate::{context::Context, Id};

use super::output_section::{OutputSection, SectionWrapper, ShareOutputSection};

pub type ShareSectionFragment = Rc<Mutex<SectionFragment>>;
#[derive(Debug)]
pub struct MergedSection {
    section: SectionWrapper,
    map: HashMap<FragmentData, ShareSectionFragment>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum FragmentData {
    Str(String),
    Constant(Vec<u8>),
}

#[derive(Debug)]
pub struct SectionFragment {
    section_id: usize,
    align: usize,
}

impl SectionFragment {
    pub fn new(id: usize, align: usize) -> Rc<Mutex<Self>> {
        Rc::new(Mutex::new(Self {
            section_id: id,
            align,
        }))
    }
}

impl OutputSection for MergedSection {
    fn is_mergeable(&self) -> bool {
        true
    }

    fn to_mergeable(&mut self) -> Option<&mut MergedSection> {
        Some(self)
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
    fn id(&self) -> usize {
        let guard = self.section.id.lock().unwrap();
        *guard
    }
    pub fn insert(&mut self, frag: &FragmentData, align: usize) -> ShareSectionFragment {
        if self.map.contains_key(frag) {
            let res = self.map[&frag].clone();
            {
                let mut guard = res.lock().unwrap();
                if guard.align < align {
                    guard.align = align;
                }
            }
            return res;
        }
        let sec_frag = SectionFragment::new(self.id(), align);
        self.map.insert(frag.clone(), sec_frag.clone());
        sec_frag
    }
}
