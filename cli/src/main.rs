use std::{fs::File, io::Write, path::Path};

use clap::Parser;

use crate::{cli::Cli, commands::Commands, config::Config};

mod cli;
mod commands;
mod config;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            if let Err(e) = init_command() {
                eprintln!("Error during init: {}", e);
            }
        },

        Commands::Run => {
            if let Err(e) = run_command() {
                eprintln!("Error during run: {}", e);
            }
        },
    }
}

const META_DIR: &str = ".mluva";
const CONFIG_FILE: &str = "mluva.yaml";
const ROOT_MODULE_DEFAULT_CONTENT: &str = r#"# This is the root module of your Mluva project.
# You can change the name of this file in the 'mluva.yaml' configuration file.
# Happy coding!

Float main() {
    return 0.0
}
"#;

fn check_if_project_initialized() -> Result<(), Box<dyn std::error::Error>> {
    let meta_dir_path = Path::new(META_DIR);
    let config_file_path = Path::new(CONFIG_FILE);
    
    if meta_dir_path.is_dir() {
        return Err(format!("Meta directory '{META_DIR}' does already exist. Aborting init...").into());
    }

    if config_file_path.is_file() {
        return Err(format!("Configuration file '{CONFIG_FILE}' does already exist. Aborting init...").into());
    }

    Ok(())
}


fn init_command() -> Result<(), Box<dyn std::error::Error>> {
    let meta_dir_path = Path::new(META_DIR);
    let config_file_path = Path::new(CONFIG_FILE);

    check_if_project_initialized()?;

    std::fs::create_dir_all(meta_dir_path)?;
    let config_file = File::create(config_file_path)?;
    let default_config = Config::default();

    serde_yaml::to_writer(config_file, &default_config)?;

    let root_module_path = Path::new(&default_config.root_module).with_extension("mv");

    if root_module_path.exists() {
        return Err(format!("Root module file '{}' does already exist.", root_module_path.display()).into());
    }
    
    let mut root_module_file = File::create(&root_module_path)?;
    root_module_file.write_all(ROOT_MODULE_DEFAULT_CONTENT.as_bytes())?;

    println!("Initialized new Mluva project:");
    println!("- Created directory '{}'", meta_dir_path.display());
    println!("- Created configuration file '{}'", config_file_path.display());
    println!("- Created root module file '{}'", root_module_path.display());

    Ok(())
}

fn run_command() -> Result<(), Box<dyn std::error::Error>> {
    
}