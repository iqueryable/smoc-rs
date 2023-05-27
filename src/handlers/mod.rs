mod setup;
mod update;

use crate::cli::Commands;

pub trait Handler {
    fn handle(&self);
}

impl Handler for Commands {
    fn handle(&self) {
        match self {
            Self::Setup {
                repository,
                template,
            } => setup::handle(repository, template),
            Self::Update {} => update::handle(),
        }
    }
}
