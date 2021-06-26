use clap::Clap;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod cli;
mod settings;
use settings::Settings;

use crate::cli::Opts;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    TermLogger::init(
        log_level(&opts),
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )?;
    log::info!("Starting up...");

    let setings = Settings::new(opts)?;
    log::info!("Setup Config {:#?}", setings);

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
