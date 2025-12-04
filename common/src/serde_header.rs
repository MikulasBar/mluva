use bincode::{Decode, Encode};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BYTECODE_MAGIC: &[u8] = &[0, 8, 29, 38, 18, 0, 8];
pub const SIGNITURE_MAGIC: &[u8] = &[0, 8, 29, 38, 35, 0, 8];

#[derive(Debug, Clone, Encode, Decode)]
pub struct SerdeHeader {
    magic: Vec<u8>,
    version: String,
}

impl SerdeHeader {
    pub fn new() -> Self {
        Self {
            magic: SIGNITURE_MAGIC.to_owned(),
            version: VERSION.to_owned(),
        }
    }

    pub fn has_valid_magic(&self) -> bool {
        self.magic == SIGNITURE_MAGIC
    }

    pub fn has_valid_version(&self) -> bool {
        self.version == VERSION
    }
}
