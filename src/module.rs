use std::collections::HashMap;

use crate::{bytecode::{write_fn_map_bytecode, BytecodeHeader, BytecodeSerializable}, compiler::{tokenize, Compiler, Parser, TypeChecker}, errors::CompileError, function::{InternalFunctionDefinition, InternalFunctionSource}};

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

    pub fn from_string(input: &str) -> Result<Self, CompileError> {
        let tokens = tokenize(input)?;
        let items = Parser::new(&tokens).parse()?;
        let (fn_map, definitions) = TypeChecker::new().check_and_return_definitions(&items)?;
        let (sources, fn_map) = Compiler::new(fn_map).compile(&items)?;
        let main_slot = fn_map.get("main").copied();

        Ok(Self {
            main_slot,
            fn_map,
            definitions, // TODO: change typechecker so it will return owned value
            sources,
        })
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


