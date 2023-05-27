mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{setup, update};

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Setup {
            repository,
            template,
        } => setup(repository, template),
        Commands::Update {} => update(),
    }
}
