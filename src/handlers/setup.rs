use std::path::Path;

use crate::{config::TemplateConfig, template::repo};

pub fn handle(repository: &String, template: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Init project from {} with template {}",
        repository, template
    );
    // ensure config does not exist
    // install templates from repository
    repo::install(&repository);
    // load template config
    let install_dir = repo::get_install_dir(&repository).unwrap();
    let template_dir = Path::new(&install_dir).join(&template);
    let template_config = TemplateConfig::load(&template_dir).unwrap();
    // create config file
    Ok(())
}
