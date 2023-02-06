use std::collections::HashMap;

use super::type_info::TypeInfo;

pub struct Compiler {
    built_ins_sign: HashMap<String, (u32, TypeInfo)>,
}

impl Compiler {
    pub fn new(info: HashMap<String, (u32, TypeInfo)>) -> Self {
        Self {
            built_ins_sign: info,
        }
    }
}
