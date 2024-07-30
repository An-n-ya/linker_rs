use crate::utils::input_file::ElfData;

pub struct Context {
    objects: Vec<ElfData>,
}

impl Context {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn push(&mut self, object: ElfData) {
        self.objects.push(object)
    }
    pub fn obj_size(&self)  -> usize{
        self.objects.len()
    }
}
