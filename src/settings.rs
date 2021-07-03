use crate::cli::Opts;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, net::SocketAddr};

#[derive(Debug, Deserialize)]
pub struct Peers {
    /// later a list of urls or ips that the application can use to discover peers
    pub address: Option<SocketAddr>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    /// Local path for sled data store
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub use_tls: bool,
    pub data: Data,
    pub peers: Peers,
    pub serve_address: Option<SocketAddr>,
}

impl Settings {
    pub fn new(opts: Opts) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Config File
        s.merge(File::with_name("appsettings"))?;

        // Development Environment Overrides
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/appsettings.{}", env)).required(false))?;

        // Environment Variables
        s.merge(Environment::with_prefix("app").separator("_"))?;

        // CLI Opts
        include_cli_opts(&mut s, opts)?;

        s.try_into()
    }
}

fn include_cli_opts(s: &mut Config, opts: Opts) -> Result<(), ConfigError> {
    if let Some(d) = opts.data_path {
        s.set("data.path", d)?;
    }

    if let Some(p) = opts.peer_addr {
        s.set("peers.address", p)?;
    }

    if let Some(srv_addr) = opts.serve_addr {
        s.set("serve_address", srv_addr)?;
    }

    Ok(())
}
