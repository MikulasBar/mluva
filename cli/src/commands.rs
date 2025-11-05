pub mod build;
pub mod init;
pub mod run;

use clap::Subcommand;

use crate::module_metadata::ModuleMetadataStorage;

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Run,
    Build,
}

const META_DIR: &str = ".mluva";
const ROOT_MODULE_DEFAULT_CONTENT: &str = r#"# This is the root module of your Mluva project.
# You can change the name of this file in the 'mluva.yaml' configuration file.

Void main() {
    print('Hello, World!')
}
"#;

fn create_meta_storage() -> Result<(), ()> {
    let dirs = [
        META_DIR.to_string(),
        ModuleMetadataStorage::MODULES_DIR.to_string(),
    ];

    let files = [ModuleMetadataStorage::FILE_PATH.to_string()];

    for dir in dirs {
        let path = std::path::Path::new(&dir);
        if !path.exists() {
            std::fs::create_dir_all(path).map_err(|e| {
                eprintln!("Failed to create directory {}: {}", dir, e);
                ()
            })?;
        }
    }

    for file in files {
        let path = std::path::Path::new(&file);
        if !path.exists() {
            std::fs::File::create(path).map_err(|e| {
                eprintln!("Failed to create file {}: {}", file, e);
                ()
            })?;
        }
    }

    Ok(())
}
