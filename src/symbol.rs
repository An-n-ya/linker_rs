use std::{rc::Rc, sync::Mutex};

pub type ShareSymbol = Rc<Mutex<Symbol>>;
#[derive(Debug)]
pub struct Symbol {
    pub elf: Option<usize>,
    pub name: String,
    pub index: usize,
    pub value: usize,
    pub is_alive: bool,
}

impl Symbol {
    pub fn new(name: String, index: usize, value: usize) -> Self {
        Self {
            name,
            index,
            value,
            elf: None,
            is_alive: true,
        }
    }
}
