use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

pub type TemplateVariables = HashMap<String, toml::Value>;

pub const CONFIG_FILE_NAME: &str = ".smoc.toml";

#[derive(Debug, Deserialize)]
pub struct TemplateConfig {
    pub vars: TemplateVariables,
}

impl TemplateConfig {
    pub fn load(folder: &Path) -> Option<Self> {
        let file = folder.join(CONFIG_FILE_NAME);
        let toml = fs::read_to_string(&file).ok()?;
        let config = match toml::from_str(&toml) {
            Ok(config) => config,
            Err(err) => {
                println!("error parsing template config: {}", err);
                return None;
            }
        };
        Some(config)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub repository: String,
    pub template: String,
    // pub revision: String,
    pub vars: TemplateVariables,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub templates: Vec<Template>,
}

impl ProjectConfig {
    pub fn load() -> Option<Self> {
        let toml = fs::read_to_string(CONFIG_FILE_NAME).ok()?;
        let config = toml::from_str(&toml).ok()?;
        Some(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let toml = toml::to_string(self)?;
        fs::write(CONFIG_FILE_NAME, toml)?;
        Ok(())
    }
}
