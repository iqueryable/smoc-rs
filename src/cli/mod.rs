use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Setup project folder
    #[command(arg_required_else_help = true)]
    Setup {
        #[arg(short, long)]
        repository: String,

        #[arg(short, long)]
        template: String,
    },
    /// Update project folder with the latest revision of the template
    Update {},
}
