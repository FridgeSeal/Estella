use clap::Clap;

/// A Dummy Application to work with Estella
#[derive(Clap)]
#[clap(name = "Estella")]
pub struct Opts {
    /// Include Debug level logs. Overrides Quiet
    #[clap(short, long)]
    pub debug: bool,

    /// Exclude all logs below Error. Overridden by Debug
    #[clap(short, long)]
    pub quiet: bool,

    /// Path to datastore
    #[clap(long)]
    pub data_path: Option<String>,

    /// Source to discover peers
    #[clap(long)]
    pub peer_src: Option<String>,
}
