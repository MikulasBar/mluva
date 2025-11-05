use super::ROOT_MODULE_DEFAULT_CONTENT;
use crate::config::Config;

pub fn command() -> Result<(), ()> {
    if Config::file_exists() {
        eprintln!(
            "Configuration file '{}' does already exist. Aborting init...",
            Config::FILE_PATH
        );
        return Err(());
    }

    let config = Config::default();
    config.save_to_file()?;

    if !config.root_module_file_exists() {
        config.save_root_module_to_file(ROOT_MODULE_DEFAULT_CONTENT)?;
    } else {
        println!(
            "Root module file '{}' already exists. Skipping creation.",
            config.root_module_file_path()
        );
    }

    println!("Initialized new Mluva project");

    Ok(())
}
