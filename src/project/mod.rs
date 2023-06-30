use std::{
    fs,
    io::{Read, Write},
    process::Command,
};

pub(crate) fn init() -> Result<(), Box<dyn std::error::Error>> {
    init_repo()?;
    create_gitignore()?;

    Ok(())
}

fn create_gitignore() -> Result<(), Box<dyn std::error::Error>> {
    let gitignore_path = ".gitignore";
    let mut content = String::new();

    match fs::File::open(&gitignore_path) {
        Ok(mut file) => {
            file.read_to_string(&mut content)?;
        }
        _ => {}
    }

    let ignore = ".smoc/";
    if !content.contains(ignore) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&gitignore_path)?;

        file.write_all(format!("{}\n{}", ignore, content).as_bytes())?;
    }

    Ok(())
}

fn init_repo() -> Result<(), Box<dyn std::error::Error>> {
    let git_dir_check = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .status()?;

    if !git_dir_check.success() {
        Command::new("git").args(["init"]).status()?;
    }

    Ok(())
}
