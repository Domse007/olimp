use std::env::Args;

use olimp::Compiler;
use olimp::Runtime;

use crate::config::Config;

pub fn run(mut args: Args, config: Config, compiler: Compiler, runtime: Runtime) -> i32 {
    let program = match args.next() {
        Some(p) => p,
        None => return 60,
    };

    0
}
