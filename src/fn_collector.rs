use std::collections::HashMap;

use crate::compiler::type_info::TypeInfo;
use crate::interpreter::stackframe::Stackframe;

#[derive(Default)]
pub struct FnCollector {
    fns: HashMap<String, (u32, Box<dyn Fn(&mut Stackframe)>, TypeInfo)>,
    counter: u32,
}

impl FnCollector {
    pub fn new() -> Self {
        Self {
            fns: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_fn<T, F>(&mut self, name: T, func: F, type_info: TypeInfo)
    where
        T: Into<String>,
        F: Fn(&mut Stackframe) + 'static,
    {
        self.fns
            .insert(name.into(), (self.counter, Box::new(func), type_info));
        self.counter += 1;
    }

    pub fn build(
        self,
    ) -> (
        HashMap<String, (u32, TypeInfo)>,
        HashMap<u32, Box<dyn Fn(&mut Stackframe)>>,
    ) {
        let mut comp_map = HashMap::new();
        let mut run_map = HashMap::new();
        for entry in self.fns {
            comp_map.insert(entry.0.to_string(), (entry.1 .0, entry.1 .2));
            run_map.insert(entry.1 .0, entry.1 .1);
        }
        (comp_map, run_map)
    }
}
