mod setup;
mod update;

use crate::cli::Commands;

pub trait Handler {
    fn handle(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Handler for Commands {
    fn handle(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Setup {
                repository,
                template,
            } => setup::handle(repository, template),
            Self::Update {} => update::handle(),
        }
    }
}
