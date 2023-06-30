use std::path::Path;

use crate::{
    config::{ProjectConfig, Template, TemplateConfig},
    project,
    template::repo,
};

pub fn handle(repository: &String, template: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Init project from {} with template {}",
        repository, template
    );
    // initialize project folder
    project::init().unwrap();
    // ensure config does not exist
    // install templates from repository
    repo::install(&repository);
    // load template config
    let install_dir = repo::get_install_dir(&repository).unwrap();
    let template_dir = Path::new(&install_dir).join(&template);
    let template_config = TemplateConfig::load(&template_dir).unwrap();
    // get existing project config or create one
    let mut config = match ProjectConfig::load() {
        Some(config) => config,
        None => ProjectConfig::default(),
    };
    // add template to project config
    config.templates.push(Template {
        repository: repository.to_owned(),
        template: template.to_owned(),
        vars: template_config.vars,
    });
    // save project config
    config.save().unwrap();
    Ok(())
}
