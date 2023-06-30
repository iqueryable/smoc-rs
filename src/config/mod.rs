use serde::Deserialize;
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
