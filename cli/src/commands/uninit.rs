use std::io::Write;

use crate::config::Config;

pub fn command() -> Result<(), Box<dyn std::error::Error>> {
    let config_file_path = std::path::Path::new(super::CONFIG_FILE);
    let Ok(config) = Config::load_from_file(config_file_path) else {
        println!("Configuration file does not exist. Cannot uninitialize.");
        return Ok(());
    };

    println!("This will permanently delete all project configuration.");
    print!(
        "To confirm, type the project name '{}': ",
        config.project_name
    );
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim() != config.project_name {
        println!("Project name mismatch. Operation cancelled.");
        return Ok(());
    }

    let meta_dir_path = std::path::Path::new(super::META_DIR);

    if meta_dir_path.is_dir() {
        std::fs::remove_dir_all(meta_dir_path)?;
    } else {
        println!(
            "Meta directory '{}' does not exist. Skipping removal.",
            meta_dir_path.display()
        );
    }

    std::fs::remove_file(config_file_path)?;
    println!("Uninitialized Mluva project.");

    Ok(())
}
