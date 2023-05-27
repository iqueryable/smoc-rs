mod cli;
mod handlers;

use clap::Parser;
use cli::Cli;
use handlers::Handler;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    cli.command.handle()
}
