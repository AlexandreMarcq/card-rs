use clap::Parser;

#[derive(Parser)]
pub enum Mode {
    /// Run the client
    Client,
    /// Start a server
    Server,
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}
