use std::collections::HashMap;

use super::error::CompilerError;
use super::type_info::TypeInfo;

pub struct Compiler {
    code: Option<String>,
    magic: Option<&'static str>,
    built_ins_sign: HashMap<String, (u32, TypeInfo)>,
}

impl Compiler {
    pub fn new(info: HashMap<String, (u32, TypeInfo)>) -> Self {
        Self {
            built_ins_sign: info,
            code: None,
            magic: None,
        }
    }

    pub fn add_code(mut self, code: String) -> Self {
        if self.code.is_some() {
            let mut string = self.code.unwrap();
            string.push_str(&code);
            self.code = Some(string);
        } else {
            self.code = Some(code);
        }
        self
    }

    pub fn set_magic(mut self, magic: &'static str) -> Self {
        self.magic = Some(magic);
        self
    }

    pub fn compile(self) -> Result<Vec<u8>, CompilerError> {
        if self.magic.is_none() {
            return Err(CompilerError::MagicNotPresent);
        }
        unimplemented!();
    }
}
