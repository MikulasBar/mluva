use std::collections::HashMap;

use crate::{
    bytecode::{write_fn_map_bytecode, BytecodeHeader, BytecodeSerializable},
    compiler::{tokenize, Compiler, Parser, TypeChecker},
    errors::{CompileError, InterpreterError},
    function::{InternalFunctionSigniture, InternalFunctionSource},
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

    pub fn from_string(input: &str) -> Result<Self, CompileError> {
        let tokens = tokenize(input)?;
        let ast = Parser::new(&tokens).parse()?;
        TypeChecker::new(&ast, &[]).check()?;
        let module = Compiler::new(ast).compile()?;

        Ok(module)
    }

    pub fn execute(&self) -> Result<(), InterpreterError> {
        todo!();
        
        // if !self.is_executable() {
        //     return Err(InterpreterError::Other(
        //         "Module is not executable (missing main function)".to_string(),
        //     ));
        // }

        // let executable_module = crate::executable_module::ExecutableModule::from_module(self)?;

        // let mut interpreter = crate::interpreter::Interpreter::new(executable_module);
        // interpreter.interpret()
    }
}

impl BytecodeSerializable for Module {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        todo!()
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
