use std::collections::HashMap;

use bincode::{
    Decode, Encode,
    error::{DecodeError, EncodeError},
};

use crate::{function::FunctionSigniture, serde_header::SerdeHeader};

#[derive(Debug, Clone, Encode, Decode)]
pub struct ModuleSigniture {
    functions: HashMap<String, FunctionSigniture>,
}

impl ModuleSigniture {
    pub fn empty() -> Self {
        Self {
            functions: HashMap::new()
        }
    }

    pub fn new(
        functions: HashMap<String, FunctionSigniture>,
    ) -> Self {
        Self {
            functions,
        }
    }

    pub fn add_function(&mut self, name: String, sig: FunctionSigniture) {
        self.functions.insert(name, sig);
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionSigniture> {
        self.functions.get(name)
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
