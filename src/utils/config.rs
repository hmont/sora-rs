use config::{Config as Cfg, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Sora {
    host: String,
    port: u16
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    host: String,
    port: u16,
    authorization: bool,
    username: Option<String>,
    password: Option<String>
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
    sora: Sora,
    database: Database
}

impl Config {
    pub fn load() -> Cfg {
        let s = Cfg::builder()
            .add_source(File::with_name("config.toml"))
            .build();

        s.expect("Failed to load config")
    }
}