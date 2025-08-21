use std::collections::HashMap;

use crate::{bytecode::header::BytecodeHeader, executable_module::ExecutableModule, function::{InternalFunctionDefinition, InternalFunctionSource}};
use serializable::BytecodeSerializable;
use bytecode_type::BytecodeType;

mod header;
mod serializable;
mod bytecode_type;


pub struct Bytecode<'a> {
    version: u8,
    bc_type: BytecodeType,
    function_map: HashMap<String, u32>,
    definitions: Vec<&'a InternalFunctionDefinition>,
    sources: Vec<InternalFunctionSource>,
}

impl<'a> Bytecode<'a> {
    pub fn from_executable(
        version: u8,
        function_map: HashMap<String, u32>,
        definitions: Vec<&'a InternalFunctionDefinition>,
        module: &'_ ExecutableModule,
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

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        self.write_bytecode(&mut buffer);
        buffer
    }

    pub fn deserialize_into_exec_module(bytes: &[u8]) -> Result<ExecutableModule, String> {
        todo!()
    }
}

impl BytecodeSerializable for Bytecode<'_> {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        todo!()
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        let mut block_buffer = vec![];

        for (name, slot) in &self.function_map {
            block_buffer.extend_from_slice(name.as_bytes());
            block_buffer.push(0); // Null terminator
            block_buffer.extend_from_slice(&slot.to_le_bytes());
        }

        for def in &self.definitions {
            def.write_bytecode(&mut block_buffer);
        }

        let header = BytecodeHeader::new(
            self.version,
            self.bc_type,
            self.function_map.len() as u32,
            block_buffer.len() as u32,
        );

        header.write_bytecode(buffer);
        buffer.extend_from_slice(&block_buffer);

        for source in &self.sources {
            source.write_bytecode(buffer);
        }
    }
}

