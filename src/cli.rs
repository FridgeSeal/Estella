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

    /// Optional address of peer node
    #[clap(long)]
    pub peer_addr: Option<String>,

    /// ipv6 address to serve on, defaults to ipv6 loopback
    #[clap(long)]
    pub serve_addr: Option<String>, // TODO: use the URL crates ability to
                                    // parse url/ip's directly
}
