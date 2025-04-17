mod compiler;
pub mod data_type;
mod data_type_scope;
mod lexer;
mod macros;
mod parser;
pub mod token;
mod type_checker;

use std::collections::HashMap;

pub use compiler::Compiler;
pub use data_type::DataType;
pub use lexer::tokenize;
pub use parser::Parser;
pub use type_checker::TypeChecker;

use crate::{
    errors::CompileError, function::ExternalFunctionSource, interpreter_source::InterpreterSource
};

pub fn compile_from_str(
    input: &str,
    externals: HashMap<String, ExternalFunctionSource>,
) -> Result<InterpreterSource, CompileError> {
    let tokens = tokenize(input)?;
    // println!("TOKENS: {:?}", tokens);
    let items = Parser::new(&tokens).parse()?;
    // println!("ITEMS: {:?}", items);
    TypeChecker::new().check(&items)?;
    Compiler::new().compile(&items, externals)
}
