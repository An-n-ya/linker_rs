use core::fmt;

pub struct StrTable {
    data: Vec<u8>,
    len: usize,
}
impl StrTable {
    pub fn new(data: Vec<u8>, len: usize) -> Self {
        Self { data, len }
    }
    pub fn get(&self, offset: usize) -> String {
        let mut res = "".to_string();
        let mut idx = offset;
        while idx < self.len {
            if self.data[idx] == 0 {
                break;
            } else {
                res.push(self.data[idx] as char);
            }
            idx += 1;
        }
        res
    }
}

impl fmt::Debug for StrTable {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.len {
            if i % 50 == 0 {
                println!();
            }
            if self.data[i] == 0 {
                print!("\\0")
            } else {
                print!("{}", self.data[i] as char)
            }
        }
        println!();
        Ok(())
    }
}
