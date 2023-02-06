use std::env::Args;
use std::fs;
use std::path::Path;

use olimp::Compiler;
use olimp::Runtime;

use crate::config::Config;

pub fn compile(mut args: Args, config: Config, compiler: Compiler, _runtime: Runtime) -> i32 {
    let filename = match args.next() {
        Some(f) => f,
        None => return 50,
    };

    let code = match fs::read_to_string(filename) {
        Ok(f) => f,
        Err(_) => return 52,
    };

    let comp_file = match args.next() {
        Some(p) => p,
        None => return 51,
    };
    let comp_file = Path::new(&comp_file);

    // FIXME: remove unwrap at some point
    let result = compiler
        .add_code(code)
        .set_magic(config.magic)
        .compile()
        .unwrap();

    fs::write(comp_file, result).unwrap();

    0
}
