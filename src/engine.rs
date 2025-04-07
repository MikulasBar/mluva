use std::collections::HashMap;

use crate::{data_type::DataType, errors::{InterpreterError, ParseError, TypeCheckError}, interpreter::Interpreter, lexer::tokenize, parser::Parser, token_tree::Stmt, type_checker::TypeChecker, value::Value};




pub struct Engine {
    functions: HashMap<String, Box<dyn ExternalFunction>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, function: impl ExternalFunction + 'static) {
        let name = function.get_name();
       self.functions.insert(name, Box::new(function));
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


pub trait ExternalFunction {
    fn get_name(&self) -> String;
    fn get_return_type(&self) -> DataType;
    fn check_types(&self, args: &[DataType]) -> Result<(), TypeCheckError>;
    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError>;
}


pub struct PrintFunction;

impl ExternalFunction for PrintFunction {
    fn get_name(&self) -> String {
        "print".to_string()
    }

    fn get_return_type(&self) -> DataType {
        DataType::Void
    }

    fn check_types(&self, _args: &[DataType]) -> Result<(), TypeCheckError> {
        Ok(())
    }

    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        for arg in args {
            println!("{}", arg);
        }
        Ok(Value::Void)
    }
}




// pub struct ExternalFunction {
//     name: String,
//     return_type: DataType,
//     check_types: fn(&[DataType]) -> Result<(), TypeCheckError>,
//     call: fn(Vec<Value>) -> Result<Value, InterpreterError>,
// } 

// const PRINT_FUNCTION: ExternalFunction = ExternalFunction {
//     name: "print".to_string(),
//     return_type: DataType::Void,
//     check_types: |args| {
//         if args.len() != 1 {
//             return Err(TypeCheckError::InvalidArgumentCount);
//         }
//         if args[0] != DataType::String {
//             return Err(TypeCheckError::InvalidArgumentType);
//         }
//         Ok(())
//     },
//     call: |args| {
//         for a in args {
//             println!("{}", a);
//         }
//         Ok(Value::Void)
//     },
// }
