use std::collections::HashMap;

use crate::{executable_module::ExecutableModule, function::{InternalFunctionDefinition, InternalFunctionSource}};
use serializable::BytecodeSerializable;
use bytecode_type::BytecodeType;

mod value;
mod instruction;
mod serializable;
mod bytecode_type;
mod function;
mod data_type;

const MAGIC: &[u8] = &[0x00, 0x08, b'm', b'v', 0x00, b'b', 0x08];

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
}

impl BytecodeSerializable for Bytecode<'_> {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        todo!()
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(MAGIC);
        buffer.push(self.version);
        self.bc_type.write_bytecode(buffer);
        let function_count = self.function_map.len() as u32;
        buffer.extend_from_slice(&function_count.to_le_bytes());

        let mut block_buffer = vec![];

        for (name, slot) in &self.function_map {
            block_buffer.extend_from_slice(name.as_bytes());
            block_buffer.push(0); // Null terminator
            block_buffer.extend_from_slice(&slot.to_le_bytes());
        }

        for def in &self.definitions {
            def.write_bytecode(&mut block_buffer);
        }

        // Offset for text section so that we can access it without loading definitions
        // the + 4 accounts for the length of this number that isn't added yet
        let text_offset = buffer.len() as u32 + block_buffer.len() as u32 + 4;
        buffer.extend_from_slice(&text_offset.to_le_bytes());

        buffer.extend_from_slice(&block_buffer);

        for source in &self.sources {
            source.write_bytecode(buffer);
        }
    }
}

