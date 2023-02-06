use std::env::Args;

use olimp::Compiler;
use olimp::Runtime;

use crate::config::Config;

pub fn help(args: Args, config: Config, _compiler: Compiler, _runtime: Runtime) -> i32 {
    0
}
