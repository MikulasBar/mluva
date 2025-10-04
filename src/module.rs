use std::collections::HashMap;

use crate::{
    bytecode::{read_fn_map_bytecode, write_fn_map_bytecode, BytecodeHeader, BytecodeSerializable},
    compiler::{tokenize, Compiler, Parser, TypeChecker},
    errors::{CompileError, RuntimeError},
    function::{InternalFunctionSigniture, InternalFunctionSource}, interpreter::Interpreter, value::Value,
};

pub struct Module {
    main_slot: Option<u32>,
    function_map: HashMap<String, u32>,
    function_signitures: Vec<InternalFunctionSigniture>,
    function_sources: Vec<InternalFunctionSource>,
}

impl Module {
    pub fn new(
        main_slot: Option<u32>,
        function_map: HashMap<String, u32>,
        function_signitures: Vec<InternalFunctionSigniture>,
        function_sources: Vec<InternalFunctionSource>,
    ) -> Self {
        Self {
            main_slot,
            function_map,
            function_signitures,
            function_sources,
        }
    }

    pub fn empty() -> Self {
        Self {
            main_slot: None,
            function_map: HashMap::new(),
            function_signitures: vec![],
            function_sources: vec![],
        }
    }

    pub fn is_executable(&self) -> bool {
        self.main_slot.is_some()
    }

    pub fn get_main_source(&self) -> Option<&InternalFunctionSource> {
        let slot = self.main_slot?;
        self.function_sources.get(slot as usize)
    }

    pub fn get_sources(&self) -> &[InternalFunctionSource] {
        &self.function_sources
    }

    pub fn get_function_signiture(&self, name: &str) -> Option<&InternalFunctionSigniture> {
        let slot = self.function_map.get(name)?;
        self.function_signitures.get(*slot as usize)
    }

    pub fn get_slot(&self, name: &str) -> Option<u32> {
        self.function_map.get(name).copied()
    }

    pub fn from_string(input: &str) -> Result<Self, CompileError> {
        let dependencies = HashMap::new();

        let tokens = tokenize(input)?;
        let ast = Parser::new(&tokens).parse()?;

        TypeChecker::new(&ast, &dependencies).check()?;
        let module = Compiler::new(ast, &dependencies).compile()?;

        Ok(module)
    }

    // TODO: rename this to something more meaningful
    pub fn from_bytecode_bytes(bytes: &[u8]) -> Result<Self, String> {
        Self::from_bytecode(bytes, &mut 0)
    }

    pub fn to_bytecode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.write_bytecode(&mut buffer);
        buffer
    }

    pub fn execute(&self) -> Result<Value, RuntimeError> {
        Interpreter::new(self).execute()
    }
}

impl BytecodeSerializable for Module {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let header = BytecodeHeader::from_bytecode(bytes, cursor)?;

        let function_map = read_fn_map_bytecode(bytes, cursor, header.function_count as usize)?;

        let mut function_signitures = Vec::with_capacity(header.function_count as usize);
        for _ in 0..header.function_count {
            function_signitures.push(InternalFunctionSigniture::from_bytecode(bytes, cursor)?);
        }

        let mut function_sources = Vec::with_capacity(header.function_count as usize);
        for _ in 0..header.function_count {
            function_sources.push(InternalFunctionSource::from_bytecode(bytes, cursor)?);
        }

        Ok(Self {
            main_slot: header.main_slot,
            function_map,
            function_signitures,
            function_sources,
        })
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        let header = BytecodeHeader::new(self.main_slot, self.function_map.len() as u32);
        header.write_bytecode(buffer);

        write_fn_map_bytecode(&self.function_map, buffer);

        for def in &self.function_signitures {
            def.write_bytecode(buffer);
        }

        for src in &self.function_sources {
            src.write_bytecode(buffer);
        }
    }
}
