use std::collections::HashMap;

use super::stackframe::Stackframe;

pub struct Runtime {
    bytes: Option<Vec<u8>>,
    magic: Option<&'static str>,
    functions: HashMap<u32, Box<dyn Fn(&mut Stackframe)>>,
}

impl Runtime {
    pub fn new(functions: HashMap<u32, Box<dyn Fn(&mut Stackframe)>>) -> Self {
        Self {
            bytes: None,
            magic: None,
            functions,
        }
    }

    pub fn set_magic(mut self, magic: &'static str) -> Self {
        self.magic = Some(magic);
        self
    }

    pub fn add_program(mut self, program: Vec<u8>) -> Self {
        self.bytes = Some(program);
        self
    }
}
