use crate::config;

pub mod repo;

pub const OUTPUT_ROOT: &str = ".smoc/output";

pub fn render(template_key: &str, repository_folder: &str, data: &config::TemplateVariables) {
    println!(
        "render_template '{}' from '{}'",
        template_key, repository_folder
    );
}
