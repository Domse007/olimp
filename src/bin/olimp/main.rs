mod compile;
mod config;
mod help;
mod run;
mod version;

use olimp::Compiler;
use olimp::OlimpBuilder;
use olimp::Runtime;

use std::collections::HashMap;
use std::env::{self, Args};

use config::Config;

fn main() {
    let mut args = env::args();

    let options = build_options();

    let config = Config::new(args.next().unwrap());

    let (compiler, runtime) = OlimpBuilder::new()
        .include_builtins(olimp::include_math)
        .magic(config.magic)
        .build();

    // FIXME: remove unwrap
    let ret_val = match options.get(args.next().unwrap().as_str()) {
        Some(func) => {
            let ret_val = func(args, config, compiler, runtime);
            assert!(ret_val != 255); // check that 255 is unique
            ret_val
        }
        None => 255,
    };

    std::process::exit(ret_val)
}

type Options = HashMap<&'static str, Box<dyn Fn(Args, Config, Compiler, Runtime) -> i32>>;

fn build_options() -> Options {
    let mut hm: Options = HashMap::new();
    hm.insert("help", Box::new(help::help));
    hm.insert("--help", Box::new(help::help));
    hm.insert("-h", Box::new(help::help));

    hm.insert("version", Box::new(version::version));
    hm.insert("--version", Box::new(version::version));
    hm.insert("-v", Box::new(version::version));

    hm.insert("compile", Box::new(compile::compile));
    hm.insert("comp", Box::new(compile::compile));

    hm.insert("run", Box::new(run::run));

    hm
}
