use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use std::{collections::HashMap, path::Path};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ModuleMetadataStorage {
    #[serde(default = "default_hashes")]
    pub map: HashMap<String, ModuleMetadata>, // source_path -> content_hash (for change detection)
}

impl ModuleMetadataStorage {
    pub const FILE_PATH: &'static str = ".mluva/modules.yaml";
    pub const MODULES_DIR: &'static str = ".mluva/modules";

    pub fn save_to_file(&self) -> Result<(), ()> {
        let path = Path::new(Self::FILE_PATH);

        let Ok(content) = serde_yaml::to_string(self) else {
            eprintln!("Failed to serialize module metadata.");
            return Err(());
        };

        let Ok(_) = std::fs::write(path, content) else {
            eprintln!(
                "Failed to write module metadata to file: {}",
                path.display()
            );
            return Err(());
        };

        Ok(())
    }

    pub fn load_from_file() -> Result<Self, ()> {
        let path = Path::new(Self::FILE_PATH);

        let Ok(content) = std::fs::read_to_string(path) else {
            eprintln!("Failed to read module metadata file: {}", path.display());
            return Err(());
        };

        let Ok(storage) = serde_yaml::from_str(&content) else {
            eprintln!("Failed to parse module metadata file: {}", path.display());
            return Err(());
        };

        Ok(storage)
    }

    /// Check if module needs recompilation
    pub fn needs_recompilation(&self, source_path: &str, content: &[u8]) -> bool {
        self.map
            .get(source_path)
            .is_none_or(|m| m.needs_recompilation(content))
    }

    /// Update hash for a module
    pub fn update_hash(&mut self, source_path: &str, content: &[u8]) {
        let hash = ModuleMetadata::calculate_content_hash(content);
        self.map.insert(
            source_path.to_string(),
            ModuleMetadata { content_hash: hash },
        );
    }
}

fn default_hashes() -> HashMap<String, ModuleMetadata> {
    HashMap::new()
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ModuleMetadata {
    pub content_hash: String,
}

impl ModuleMetadata {
    fn calculate_content_hash(content: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn needs_recompilation(&self, content: &[u8]) -> bool {
        let current_hash = Self::calculate_content_hash(content);
        self.content_hash != current_hash
    }

    pub fn source_to_bytecode_path(source_path: &str) -> String {
        format!(
            "{}/{}.mvb",
            ModuleMetadataStorage::MODULES_DIR,
            Self::encode_path(source_path)
        )
    }

    fn encode_path(source_path: &str) -> String {
        let normalized = source_path.replace('\\', "/");
        URL_SAFE_NO_PAD.encode(normalized.as_bytes())
    }
}
