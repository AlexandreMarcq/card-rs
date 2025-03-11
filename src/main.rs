use clap::Parser;
use cli::Cli;

mod cli;
mod client;
mod server;
mod uno;

fn main() {
    let cli = Cli::parse();

    match &cli.mode {
        cli::Mode::Client => client::run(),
        cli::Mode::Server => server::run(),
    }
}
