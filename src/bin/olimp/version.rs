use std::env::Args;

use crate::config::Config;

pub fn version(_args: Args, config: Config) -> i32 {
    println!(
        "Name: {} (called as {})",
        config.pkg_name, config.invoc_name
    );
    println!("Version: {}", config.version);
    println!("Magic: {}", config.magic);

    0
}
