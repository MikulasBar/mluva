use std::collections::HashMap;

use crate::{executable_module::ExecutableModule, function::{InternalFunctionDefinition, InternalFunctionSource}};
use serializable::BytecodeSerializable;
use bytecode_type::BytecodeType;

mod value;
mod instruction;
mod serializable;
mod bytecode_type;

const MAGIC: &[u8] = b"\0mluvabc";

pub struct Bytecode {
    version: u8,
    bc_type: BytecodeType,
    function_map: HashMap<String, u32>,
    definitions: Vec<InternalFunctionDefinition>,
    sources: Vec<InternalFunctionSource>,
}

impl Bytecode {
    pub fn from_executable(
        version: u8,
        function_map: HashMap<String, u32>,
        definitions: Vec<InternalFunctionDefinition>,
        module: ExecutableModule,
    ) -> Self {
        let sources = module.functions.iter().map(|f| f.clone()).collect();
        Bytecode {
            version,
            bc_type: BytecodeType::Executable { main_slot: module.main_slot  as u32 },
            function_map,
            definitions,
            sources,
        }
    }
}

impl BytecodeSerializable for Bytecode {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        todo!()
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        
    }
}

