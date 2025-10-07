
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
            root_module: "main".to_string(),
        }
    }
}

fn default_root_module() -> String {
    "main".to_string()
}