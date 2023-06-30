use std::{fs, process::Command};

use crate::{
    config::ProjectConfig,
    template::{self, repo},
};

pub fn handle() -> Result<(), Box<dyn std::error::Error>> {
    println!("Update project");
    // ensure config exists
    let config = ProjectConfig::load().unwrap();
    // ensure no changes in the working directory
    // initialize output directory
    fs::create_dir_all(template::OUTPUT_ROOT).unwrap();
    Command::new("git")
        .args(["init"])
        .current_dir(template::OUTPUT_ROOT)
        .status()
        .unwrap();
    // map templates variables
    // render the current revision if any
    // render latest revision
    for template_config in config.templates.iter() {
        let repository = &template_config.repository;
        let template = &template_config.template;
        repo::install(repository);
        let repo_folder = repo::get_install_dir(repository).unwrap();
        template::render(template, &repo_folder, &template_config.vars)
    }
    // update config file
    // merge change revision into working directory
    Ok(())
}
