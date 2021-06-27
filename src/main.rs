use blip::Mesh;
use clap::Clap;
use log::info;
use rand::Rng;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use std::net::SocketAddr;

mod cli;
mod settings;
use cli::Opts;
use settings::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    TermLogger::init(
        log_level(&opts),
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )?;
    let config = Settings::new(opts)?;
    log::info!("Setup Config {:#?}", config);
    info!("Setup Config. Starting mesh");
    let mesh = Mesh::low_latency();
    info!("Low-latency-mesh setup");
    let listen_addr = {
        let mut rng = rand::thread_rng();
        let mut default_serve_adr: SocketAddr = "[::1]:1234".parse()?;
        default_serve_adr.set_port(rng.gen_range(1024..65000));
        default_serve_adr
    };
    log::info!("Listening on port: {}", listen_addr.port());
    if let Some(peer_sock_addr) = config.peers.address {
        mesh.join_seed(peer_sock_addr, false)
            .serve(listen_addr)
            .await?;
    } else {
        log::info!("No peer node provided to bootstrap from: starting as single node");
        mesh.serve(listen_addr).await?;
    };
    Ok(())
}

fn log_level(opts: &Opts) -> LevelFilter {
    if opts.debug {
        LevelFilter::Debug
    } else if opts.quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    }
}
