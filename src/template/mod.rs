use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{config, template::engine::Engine};

mod engine;
mod helpers;
pub mod repo;

pub const OUTPUT_ROOT: &str = ".smoc/output";

fn find_files(root_path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();

    if root_path.is_dir() {
        for entry in fs::read_dir(root_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let subdirectory_files = find_files(&path)?;
                files.extend(subdirectory_files);
            } else {
                files.push(path);
            }
        }
    }

    Ok(files)
}

fn create_directories(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parent_dir = Path::new(file_path).parent().unwrap();
    fs::create_dir_all(parent_dir)?;
    Ok(())
}

fn has_file_extension(file_path: &Path, extension: &str) -> bool {
    if let Some(file_extension) = file_path.extension() {
        println!("file_extension: {:?}", file_extension);
        file_extension == extension
    } else {
        false
    }
}

pub fn render(template_key: &str, repository_folder: &str, data: &config::TemplateVariables) {
    println!(
        "render_template '{}' from '{}'",
        template_key, repository_folder
    );
    let mut engine = Engine::new();

    // find all files in directory
    let root_path = Path::new(repository_folder).join(template_key);
    let files = find_files(&root_path).unwrap();

    for file in files {
        let file_path = Path::new(file.to_str().unwrap());
        let base_dir = Path::new(root_path.to_str().unwrap());
        let relative_path = Path::new(OUTPUT_ROOT).join(file_path.strip_prefix(base_dir).unwrap());

        let path = relative_path.to_str().unwrap();
        let modified_path = path.replace("[[", "{{").replace("]]", "}}");
        let rendered_target_path = engine.render_path(&modified_path, data).unwrap();

        create_directories(&rendered_target_path).unwrap();

        if has_file_extension(Path::new(&rendered_target_path), "smoc") {
            engine
                .render_file(
                    &file_path.to_str().unwrap(),
                    &rendered_target_path.trim_end_matches(".smoc"),
                    data,
                )
                .unwrap();
        } else {
            match rendered_target_path.ends_with(config::CONFIG_FILE_NAME) {
                true => {}
                false => {
                    fs::copy(file_path, rendered_target_path).unwrap();
                }
            }
        }
    }
}
