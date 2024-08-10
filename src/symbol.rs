use std::{rc::Rc, sync::Mutex};

use crate::{output_section::merged_section::ShareSectionFragment, section::Section};

pub type ShareSymbol = Rc<Mutex<Symbol>>;
#[derive(Debug)]
pub struct Symbol {
    pub elf: Option<usize>,
    pub name: String,
    pub index: usize,
    pub value: usize,
    input_section: Option<Section>,
    frag: Option<ShareSectionFragment>,
    pub is_alive: bool,
}

impl Symbol {
    pub fn new(name: String, index: usize, value: usize) -> Self {
        Self {
            name,
            index,
            value,
            elf: None,
            input_section: None,
            frag: None,
            is_alive: true,
        }
    }
    pub fn set_frag(&mut self, frag: ShareSectionFragment) {
        self.input_section = None;
        self.frag = Some(frag);
    }
    pub fn set_section(&mut self, section: Section) {
        // TODO: handle input section
        self.input_section = Some(section);
        self.frag = None;
    }
}
