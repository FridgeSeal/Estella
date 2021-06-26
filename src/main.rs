use anyhow;
use clap::Clap;
use log;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod cli;
mod settings;
use settings::Settings;

use crate::cli::Opts;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let setings = Settings::new(opts)?;

    TermLogger::init(
        log_level(&setings),
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )?;
    log::info!("Starting up...");

    log::info!("Setup Config {:#?}", setings);

    Ok(())
}

fn log_level(settings: &Settings) -> LevelFilter {
    if settings.debug {
        LevelFilter::Debug
    } else if settings.quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    }
}
