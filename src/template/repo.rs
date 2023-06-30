use std::{fs, path::PathBuf, process::Command};

const INSTALL_ROOT: &str = ".smoc/repositories";

pub(crate) fn get_install_dir(repository: &str) -> Option<String> {
    let name = get_name(repository);
    PathBuf::from(INSTALL_ROOT)
        .join(name)
        .to_str()
        .map(String::from)
}

pub(crate) fn get_name(repository: &str) -> &str {
    repository
        .split('/')
        .last()
        .unwrap()
        .trim_end_matches(".git")
}

pub(crate) fn install(repository: &str) {
    println!("Install templates from {}", repository);
    let install_dir = get_install_dir(repository).unwrap();
    fs::remove_dir_all(&install_dir).ok();
    Command::new("git")
        .args(["clone", repository, &install_dir])
        .output()
        .unwrap();
}
