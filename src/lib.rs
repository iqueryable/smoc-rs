mod cli;
mod handlers;

use clap::Parser;
use cli::Cli;
use handlers::Handler;

pub fn run() {
    let cli = Cli::parse();
    cli.command.handle();
}
