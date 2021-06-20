use anyhow;
use log;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

mod settings;
use settings::Settings;

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )?;
    log::info!("Starting up...");
    Settings::new()?;
    log::info!("Setup Config");
    Ok(())
}
