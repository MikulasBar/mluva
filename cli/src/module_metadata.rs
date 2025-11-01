use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ModuleMetadata {
    #[serde(default = "default_hashes")]
    pub hashes: HashMap<String, String>, // source_path -> content_hash (for change detection)
}

impl ModuleMetadata {
    pub fn new() -> Self {
        Self {
            hashes: HashMap::new(),
        }
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load hashes from a YAML file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let hashes: Self = serde_yaml::from_str(&content)?;
        Ok(hashes)
    }

    /// Convert bytecode filename back to source path
    pub fn bytecode_filename_to_source(
        filename: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let encoded = filename
            .strip_suffix(".mvb")
            .ok_or("Invalid bytecode filename: missing .mvb extension")?;
        Self::decode_path(encoded)
    }

    /// Check if module needs recompilation
    pub fn needs_recompilation(&self, source_path: &str, content: &[u8]) -> bool {
        let current_hash = Self::calculate_content_hash(content);
        self.hashes.get(source_path) != Some(&current_hash)
    }

    /// Update hash for a module
    pub fn update_hash(&mut self, source_path: &str, content: &[u8]) {
        let hash = Self::calculate_content_hash(content);
        self.hashes.insert(source_path.to_string(), hash);
    }

    /// Calculate content hash for change detection
    fn calculate_content_hash(content: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Convert source path to bytecode filename
    pub fn source_to_bytecode_filename(source_path: &str) -> String {
        format!("{}.mvb", Self::encode_path(source_path))
    }
    /// Encode a file path to a safe filename using base64
    fn encode_path(source_path: &str) -> String {
        let normalized = source_path.replace('\\', "/");
        URL_SAFE_NO_PAD.encode(normalized.as_bytes())
    }

    /// Decode a base64 filename back to the original path
    fn decode_path(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = URL_SAFE_NO_PAD.decode(encoded)?;
        Ok(String::from_utf8(bytes)?)
    }
}

fn default_hashes() -> HashMap<String, String> {
    HashMap::new()
}
