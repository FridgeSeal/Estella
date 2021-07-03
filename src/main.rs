use clap::Clap;
use evmap;
use log::info;
use rand::Rng;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use tonic;
mod cli;
mod membership_mesh;
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
    let (handle, hmap) = membership_mesh::boostrap(&config).await?;
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
