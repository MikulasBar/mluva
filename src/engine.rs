use crate::{
    errors::{CompileError, InterpreterError},
    external::ExternalFunction,
    function_table::FunctionTable,
    instruction::Instruction,
    interpreter::Interpreter,
    compiler::{tokenize, Compiler, Parser, TypeChecker},
};

pub struct Engine {
    function_table: FunctionTable,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            function_table: FunctionTable::new(),
        }
    }

    pub fn add_function(&mut self, function: ExternalFunction) {
        self.function_table.insert(function);
    }

    pub fn compile(&self, input: &str) -> Result<(Vec<Instruction>, usize), CompileError> {
        let tokens = tokenize(input)?;
        let stmts = Parser::new(&tokens).parse()?;
        TypeChecker::new(&self.function_table).check(&stmts)?;

        let compiler = Compiler::new(&self.function_table);
        let compile_result = compiler.compile(&stmts);

        Ok(compile_result)
    }

    pub fn interpret(
        &self,
        instructions: Vec<Instruction>,
        slot_used: usize,
    ) -> Result<(), InterpreterError> {
        let mut interpreter = Interpreter::new(&self.function_table, instructions, slot_used);
        interpreter.interpret()
    }
}
