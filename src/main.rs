use clap::{Parser, Subcommand};
use commands::{setup, update};

pub mod commands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Initialize project folder
    Init {
        #[arg(short, long)]
        repository: String,

        #[arg(short, long)]
        template: String,
    },
    /// Update project folder with the latest revision of the template
    Update {},
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        SubCommand::Init {
            repository,
            template,
        } => setup(repository, template),
        SubCommand::Update {} => update(),
    }
}
