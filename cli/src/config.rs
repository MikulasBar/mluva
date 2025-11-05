use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project_name: String,

    #[serde(default = "default_root_module", skip_serializing)]
    pub root_module: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            project_name: "My Mluva Project".to_string(),
            root_module: default_root_module(),
        }
    }
}

impl Config {
    pub const FILE_PATH: &'static str = "mluva.yaml";

    pub fn file_exists() -> bool {
        if Path::new(Self::FILE_PATH).is_file() {
            true
        } else {
            false
        }
    }

    pub fn load_from_file() -> Result<Self, ()> {
        let path = Path::new(Self::FILE_PATH);

        let Ok(file) = std::fs::File::open(path) else {
            eprintln!("Failed to open config file: {}", path.display());
            return Err(());
        };

        let Ok(config) = serde_yaml::from_reader(file) else {
            eprintln!("Failed to parse config file: {}", path.display());
            return Err(());
        };

        Ok(config)
    }

    pub fn save_to_file(&self) -> Result<(), ()> {
        let path = Path::new(Self::FILE_PATH);

        let Ok(file) = std::fs::File::create(path) else {
            eprintln!("Failed to create config file: {}", path.display());
            return Err(());
        };

        let Ok(_) = serde_yaml::to_writer(file, self) else {
            eprintln!("Failed to write config file: {}", path.display());
            return Err(());
        };

        Ok(())
    }

    pub fn root_module_file_path(&self) -> String {
        let root_module_path = Path::new(&self.root_module).with_extension("mv");
        root_module_path.to_string_lossy().to_string()
    }

    pub fn save_root_module_to_file(&self, source: &str) -> Result<(), ()> {
        let root_module_path = Path::new(&self.root_module).with_extension("mv");

        let Ok(_) = std::fs::write(&root_module_path, source) else {
            eprintln!(
                "Failed to write root module file: {}",
                root_module_path.display()
            );
            return Err(());
        };

        Ok(())
    }

    pub fn root_module_file_exists(&self) -> bool {
        let root_module_path = Path::new(&self.root_module).with_extension("mv");
        root_module_path.is_file()
    }
}

fn default_root_module() -> String {
    "main".to_string()
}
