use crate::compiler::compiler::Compiler;
use crate::fn_collector::FnCollector;
use crate::interpreter::runtime::{self, Runtime};

#[derive(Default)]
pub struct OlimpBuilder {
    magic: [u8; 7],
    included_builtins: FnCollector,
}

impl OlimpBuilder {
    pub fn new() -> Self {
        Self {
            magic: Default::default(),
            included_builtins: FnCollector::default(),
        }
    }

    pub fn magic<T>(mut self, str: T) -> Self
    where
        T: Into<String>,
    {
        let mut str = str.into();
        str.truncate(7);
        let mut str = str.bytes();
        let magic = [
            str.next().unwrap(),
            str.next().unwrap(),
            str.next().unwrap(),
            str.next().unwrap(),
            str.next().unwrap(),
            str.next().unwrap(),
            str.next().unwrap(),
        ];
        self.magic = magic;

        self
    }

    pub fn include_builtins<F: Fn(&mut FnCollector)>(mut self, collection: F) -> OlimpBuilder {
        collection(&mut self.included_builtins);
        self
    }

    pub fn build(self) -> (Compiler, Runtime) {
        let (comp, run) = self.included_builtins.build();
        let compiler = Compiler::new(comp);
        let runtime = Runtime::new(run);
        (compiler, runtime)
    }
}

#[test]
fn test_builder_include_builtins() {
    use crate::interpreter::builtins::math::include_math;

    let ob = OlimpBuilder::new().include_builtins(include_math);
}
