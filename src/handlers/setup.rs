pub fn handle(repository: &String, template: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Init project from {} with template {}",
        repository, template
    );
    // ensure config does not exist
    // install templates from repository
    // create config file
    Ok(())
}
