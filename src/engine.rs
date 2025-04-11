use crate::{
    compiler::{tokenize, Compiler, Parser, TypeChecker},
    errors::{CompileError, InterpreterError},
    external::ExternalFunction,
    function_table::FunctionTable,
    interpreter::Interpreter,
    interpreter_source::InterpreterSource,
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

    pub fn compile(&self, input: &str) -> Result<InterpreterSource, CompileError> {
        let tokens = tokenize(input)?;
        let stmts = Parser::new(&tokens).parse()?;
        println!("{:?}", stmts);
        TypeChecker::new(&self.function_table).check(&stmts)?;

        let compile_result = Compiler::new(&self.function_table).compile(&stmts);

        Ok(compile_result)
    }

    pub fn interpret(&self, source: InterpreterSource) -> Result<(), InterpreterError> {
        let mut interpreter = Interpreter::new(&self.function_table, source);
        interpreter.interpret()
    }
}
