use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub data_path: String, // Local path for sled data store
    pub serve_path: String,
    pub serve_port: u16,
    pub peer_discovery_src: String, // later a list of urls or ips that the application can use to discover peers
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config.yaml"))?;
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        s.merge(Environment::with_prefix("app"))?;
        s.try_into()
    }
}
