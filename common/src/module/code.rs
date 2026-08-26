use std::collections::HashMap;

use bincode::{
    Decode, Encode,
    error::{DecodeError, EncodeError},
};

use crate::{function::FunctionCode, module::lcp::LCP, serde_header::SerdeHeader};

#[derive(Debug, Clone, Encode, Decode)]
pub struct ModuleCode {
    lcp: LCP,
    functions: HashMap<String, FunctionCode>,
}

impl ModuleCode {
    pub fn empty() -> Self {
        Self {
            lcp: LCP::new(),
            functions: HashMap::new(),
        }
    }

    pub fn new(lcp: LCP, functions: HashMap<String, FunctionCode>) -> Self {
        Self { lcp, functions }
    }

    pub fn get_main_code(&self) -> Option<&FunctionCode> {
        self.functions.get("main")
    }

    pub fn get_code(&self, name: &str) -> Option<&FunctionCode> {
        self.functions.get(name)
    }

    pub fn add_code(&mut self, name: String, code: FunctionCode) {
        self.functions.insert(name, code);
    }

    pub fn get_lcp_mut(&mut self) -> &'_ mut LCP {
        &mut self.lcp
    }

    pub fn serialize(&self) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec((SerdeHeader::new(), &self), bincode::config::standard())
    }

    pub fn deserialize(src: &[u8]) -> Result<Self, DecodeError> {
        let ((header, module), _count): ((SerdeHeader, Self), usize) =
            bincode::decode_from_slice(src, bincode::config::standard())?;

        if !header.has_valid_magic() {
            return Err(DecodeError::OtherString(
                "Invalid bytecode magic".to_string(),
            ));
        }

        if !header.has_valid_version() {
            return Err(DecodeError::OtherString(
                "Incompatible bytecode version".to_string(),
            ));
        }

        Ok(module)
    }
}
