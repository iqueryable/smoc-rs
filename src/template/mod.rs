use crate::config;

mod engine;
mod helpers;
pub mod repo;

pub const OUTPUT_ROOT: &str = ".smoc/output";

pub fn render(template_key: &str, repository_folder: &str, data: &config::TemplateVariables) {
    println!(
        "render_template '{}' from '{}'",
        template_key, repository_folder
    );
    let mut engine = Engine::new();
}
