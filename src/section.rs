use std::{rc::Rc, sync::Mutex};

use crate::{context::Context, linker::SectionFlag};

pub type ShareSection = Rc<Mutex<Section>>;
#[derive(Debug)]
pub struct Section {
    pub elf: usize,
    pub name: String,
    pub index: usize,
    pub data: Vec<u8>,
}

impl Section {
    pub fn is_write(&self, ctx: &Context) -> bool {
        if let Some(elf) = ctx.get_object(self.elf) {
            let elf = elf.lock().unwrap();
            if elf.section_info.elf_sections[self.index].flags & SectionFlag::WRITE as u64 != 0 {
                return true;
            }
        }
        false
    }
    pub fn is_alloc(&self, ctx: &Context) -> bool {
        if let Some(elf) = ctx.get_object(self.elf) {
            let elf = elf.lock().unwrap();
            if elf.section_info.elf_sections[self.index].flags & SectionFlag::ALLOC as u64 != 0 {
                return true;
            }
        }
        false
    }
    pub fn is_merge(&self, ctx: &Context) -> bool {
        if let Some(elf) = ctx.get_object(self.elf) {
            let elf = elf.lock().unwrap();
            if elf.section_info.elf_sections[self.index].flags & SectionFlag::MERGE as u64 != 0 {
                return true;
            }
        }
        false
    }
    pub fn is_string(&self, ctx: &Context) -> bool {
        if let Some(elf) = ctx.get_object(self.elf) {
            let elf = elf.lock().unwrap();
            if elf.section_info.elf_sections[self.index].flags & SectionFlag::STRINGS as u64 != 0 {
                return true;
            }
        }
        false
    }
}
