use std::collections::HashMap;

use crate::{
    errors::{InterpreterError, ParseError, TypeCheckError},
    external::ExternalFunction,
    interpreter::Interpreter,
    lexer::tokenize,
    parser::Parser,
    token_tree::Stmt,
    type_checker::TypeChecker,
};

pub struct Engine {
    functions: HashMap<&'static str, ExternalFunction>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, function: ExternalFunction) {
        let name = function.name;
        self.functions.insert(name, function);
    }

    pub fn parse(&self, input: &str) -> Result<Vec<Stmt>, ParseError> {
        let tokens = tokenize(input)?;
        Parser::new(&tokens).parse()
    }

    pub fn type_check(&self, stmts: &[Stmt]) -> Result<(), TypeCheckError> {
        let mut type_checker = TypeChecker::new(&self.functions);
        type_checker.check(stmts)
    }

    pub fn interpret(&self, stmts: &[Stmt]) -> Result<(), InterpreterError> {
        let mut interpreter = Interpreter::new(&self.functions);
        interpreter.interpret(stmts)
    }
}
