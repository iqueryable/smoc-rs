use std::{fs, process::Command};

use crate::{config::ProjectConfig, template};

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
    // run templates in temporary repository
    // update config file
    // merge change revision into working directory
    Ok(())
}
