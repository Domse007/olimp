use crate::compiler::compiler::Compiler;
use crate::fn_collector::FnCollector;
use crate::interpreter::runtime::{self, Runtime};

#[derive(Default)]
pub struct OlimpBuilder {
    magic: [u8; 7],
    included_builtins: FnCollector,
    exec_bytes: Option<Vec<u8>>,
}

impl OlimpBuilder {
    pub fn new() -> Self {
        Self {
            magic: Default::default(),
            included_builtins: FnCollector::default(),
            exec_bytes: None,
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

    pub fn add_program(mut self, bytes: &[u8]) -> Self {
        self.exec_bytes = Some(bytes.into());
        self
    }

    pub fn build(self) -> (Compiler, Option<Runtime>) {
        let (comp, run) = self.included_builtins.build();
        let compiler = Compiler::new(comp);
        let runtime = match &self.exec_bytes {
            Some(b) => Some(Runtime::new(&self.exec_bytes.unwrap(), run)),
            None => None,
        };
        (compiler, runtime)
    }
}

#[test]
fn test_builder_include_builtins() {
    use crate::interpreter::builtins::math::include_math;

    let ob = OlimpBuilder::new().include_builtins(include_math);
}
