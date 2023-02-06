use std::collections::HashMap;

use super::stackframe::Stackframe;

pub struct Runtime {
    bytes: Vec<u8>,
    functions: HashMap<u32, Box<dyn Fn(&mut Stackframe)>>,
}

impl Runtime {
    pub fn new(bytes: &[u8], functions: HashMap<u32, Box<dyn Fn(&mut Stackframe)>>) -> Self {
        Self {
            bytes: bytes.into(),
            functions,
        }
    }
}
