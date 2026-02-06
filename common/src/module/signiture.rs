use std::collections::HashMap;

use bincode::{
    Decode, Encode,
    error::{DecodeError, EncodeError},
};

use crate::{function::FunctionSigniture, serde_header::SerdeHeader};

#[derive(Debug, Clone, Encode, Decode)]
pub struct ModuleSigniture {
    dependency_pool: Vec<String>,
    fn_slots: HashMap<String, u32>,
    signitures: Vec<FunctionSigniture>,
}

impl ModuleSigniture {
    pub fn empty() -> Self {
        Self {
            dependency_pool: vec![],
            fn_slots: HashMap::new(),
            signitures: vec![],
        }
    }

    pub fn new(
        dependency_pool: Vec<String>,
        fn_slots: HashMap<String, u32>,
        signitures: Vec<FunctionSigniture>,
    ) -> Self {
        Self {
            dependency_pool,
            fn_slots,
            signitures,
        }
    }

    pub fn add_dependency(&mut self, dependency: String) -> u32 {
        let slot = self.dependency_pool.len() as u32;
        self.dependency_pool.push(dependency);
        slot
    }

    pub fn add_fn(&mut self, name: String, signiture: FunctionSigniture) -> u32 {
        let slot = self.signitures.len() as u32;
        self.signitures.push(signiture);
        self.fn_slots.insert(name, slot);
        slot
    }

    pub fn get_dependency(&self, slot: u32) -> Option<&String> {
        self.dependency_pool.get(slot as usize)
    }

    pub fn get_sign(&self, slot: u32) -> Option<&FunctionSigniture> {
        self.signitures.get(slot as usize)
    }

    pub fn get_sign_by_name(&self, name: &str) -> Option<&FunctionSigniture> {
        let slot = self.fn_slots.get(name)?;
        self.signitures.get(*slot as usize)
    }

    pub fn get_fn_slot(&self, name: &str) -> Option<u32> {
        self.fn_slots.get(name).copied()
    }

    pub fn get_fn_map(&self) -> &HashMap<String, u32> {
        &self.fn_slots
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
