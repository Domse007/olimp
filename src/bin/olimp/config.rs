use olimp::{MAGIC, VERSION};

pub struct Config {
    pub version: &'static str,
    pub magic: &'static str,
    pub pkg_name: &'static str,
    pub invoc_name: String,
}

impl Config {
    pub fn new(invoc_name: String) -> Self {
        Self {
            version: VERSION,
            magic: MAGIC,
            pkg_name: env!("CARGO_PKG_NAME"),
            invoc_name,
        }
    }
}
