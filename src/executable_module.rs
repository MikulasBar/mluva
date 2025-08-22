use crate::{
    bytecode::{BytecodeHeader, BytecodeSerializable as _, BytecodeType},
    function::InternalFunctionSource,
};

#[derive(Debug, Clone)]
pub struct ExecutableModule {
    pub functions: Vec<InternalFunctionSource>,
    pub main_slot: u32,
}

impl ExecutableModule {
    pub fn new(functions: Vec<InternalFunctionSource>, main_slot: u32) -> Self {
        Self {
            functions,
            main_slot,
        }
    }

    pub fn from_bytecode(bytes: &[u8]) -> Result<Self, String> {
        let mut cursor = 0;
        let header = BytecodeHeader::from_bytecode(bytes, &mut cursor)?;

        let BytecodeType::Executable { main_slot } = header.bc_type else {
            return Err("Expected Executable bytecode".to_string());
        };

        // println!("DE: Cursor after header: {}", cursor);
        // println!("DE: Text block offset: {}", header.text_block_offset);
        cursor = header.text_block_offset as usize;

        let mut functions = Vec::with_capacity(header.function_count as usize);

        for _ in 0..header.function_count {
            let f = InternalFunctionSource::from_bytecode(bytes, &mut cursor)?;
            functions.push(f);
        }

        Ok(ExecutableModule::new(functions, main_slot))
    }
}
