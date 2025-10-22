/// Parse command line arguments
pub(crate) fn parse() -> Args {
    use clap::Parser;
    Args::parse()
}

#[derive(clap::Parser, Debug)]
#[command(
    version = "0.1.0",
    about = "A simple web service for temporarily storing text or files.",
    long_about = None
)]
pub(crate) struct Args {
    /// Network interface to bind the server to
    ///
    /// Use 0.0.0.0 to listen on all interfaces; use 127.0.0.1 for localhost-only.
    #[arg(short = 'H', long, default_value = "0.0.0.0")]
    pub(crate) host: String,

    /// Port the server listens on
    #[arg(short, long, default_value_t = 8765)]
    pub(crate) port: u16,

    /// Enable verbose debug logging
    #[arg(long)]
    pub(crate) debug: bool,

    /// Path to the directory where program's data are stored
    ///
    /// Defaults to the program's directory.
    #[arg(long)]
    pub(crate) data_dir: Option<String>,

    /// Cabinet number
    #[arg(long, default_value_t = 100)]
    pub(crate) cabinet_number: i64,
}
