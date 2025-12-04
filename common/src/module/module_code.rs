use bincode::{
    Decode, Encode,
    error::{DecodeError, EncodeError},
};

use crate::{function_code::FunctionCode, serde_header::SerdeHeader};

#[derive(Debug, Clone, Encode, Decode)]
pub struct ModuleCode {
    string_pool: Vec<String>,
    main_slot: Option<u32>,
    function_codes: Vec<FunctionCode>,
}

impl ModuleCode {
    pub fn empty() -> Self {
        Self {
            string_pool: vec![],
            main_slot: None,
            function_codes: vec![],
        }
    }

    pub fn new(
        string_pool: Vec<String>,
        main_slot: Option<u32>,
        function_codes: Vec<FunctionCode>,
    ) -> Self {
        Self {
            string_pool,
            main_slot,
            function_codes,
        }
    }

    pub fn is_executable(&self) -> bool {
        self.main_slot.is_some()
    }

    pub fn set_main_slot(&mut self, slot: Option<u32>) {
        self.main_slot = slot;
    }

    pub fn get_main_slot(&self) -> Option<u32> {
        self.main_slot
    }

    pub fn get_main_code(&self) -> Option<&FunctionCode> {
        let slot = self.main_slot?;
        self.function_codes.get(slot as usize)
    }

    pub fn get_codes(&self) -> &[FunctionCode] {
        &self.function_codes
    }

    pub fn get_code_by_slot(&self, slot: u32) -> Option<&FunctionCode> {
        self.function_codes.get(slot as usize)
    }

    pub fn get_string_from_pool(&self, slot: u32) -> Option<&str> {
        self.string_pool.get(slot as usize).map(|s| s.as_str())
    }

    pub fn add_string(&mut self, s: String) -> u32 {
        let slot = self.string_pool.len() as u32;
        self.string_pool.push(s);
        slot
    }

    pub fn add_code(&mut self, code: FunctionCode) -> u32 {
        let slot = self.function_codes.len() as u32;
        self.function_codes.push(code);
        slot
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
