use std::{fs::File, io::Write as _, path::Path};

use crate::config::Config;

use super::{CONFIG_FILE, META_DIR, MODULES_META_FILE, ROOT_MODULE_DEFAULT_CONTENT};

pub fn command() -> Result<(), Box<dyn std::error::Error>> {
    let meta_dir_path = Path::new(META_DIR);
    let config_file_path = Path::new(CONFIG_FILE);

    if meta_dir_path.is_dir() {
        return Err(
            format!("Meta directory '{META_DIR}' does already exist. Aborting init...").into(),
        );
    }

    if config_file_path.is_file() {
        return Err(format!(
            "Configuration file '{CONFIG_FILE}' does already exist. Aborting init..."
        )
        .into());
    }

    std::fs::create_dir_all(meta_dir_path)?;
    std::fs::create_dir_all(meta_dir_path.join("modules"))?;
    let _ = File::create(meta_dir_path.join(MODULES_META_FILE))?;
    let config_file = File::create(config_file_path)?;
    let default_config = Config::default();

    serde_yaml::to_writer(config_file, &default_config)?;

    let root_module_path = Path::new(&default_config.root_module).with_extension("mv");

    if !root_module_path.exists() {
        let mut root_module_file = File::create(&root_module_path)?;
        root_module_file.write_all(ROOT_MODULE_DEFAULT_CONTENT.as_bytes())?;
    } else {
        println!(
            "Root module file '{}' already exists. Skipping creation.",
            root_module_path.display()
        );
    }

    println!("Initialized new Mluva project");

    Ok(())
}
