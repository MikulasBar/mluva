use std::collections::HashMap;

use crate::{bytecode::{write_fn_map_bytecode, BytecodeHeader, BytecodeSerializable}, function::{InternalFunctionDefinition, InternalFunctionSource}};

pub struct Module {
    main_slot: Option<u32>,
    fn_map: HashMap<String, u32>,
    definitions: Vec<InternalFunctionDefinition>,
    sources: Vec<InternalFunctionSource>,
}

impl Module {
    pub fn empty() -> Self {
        Self {
            main_slot: None,
            fn_map: HashMap::new(),
            definitions: vec![],
            sources: vec![],
        }
    }

    pub fn is_executable(&self) -> bool {
        self.main_slot.is_some()
    }
}


impl BytecodeSerializable for Module {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        todo!()
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        let header = BytecodeHeader::new(self.main_slot, self.fn_map.len() as u32);
        header.write_bytecode(buffer);
        
        write_fn_map_bytecode(&self.fn_map, buffer);

        for def in &self.definitions {
            def.write_bytecode(buffer);
        }

        for src in &self.sources {
            src.write_bytecode(buffer);
        }
    }
}


