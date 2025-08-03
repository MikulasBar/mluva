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

use crate::{errors::CompileError, executable_module::ExecutableModule};

pub fn compile_from_str(input: &str) -> Result<ExecutableModule, CompileError> {
    let tokens = tokenize(input)?;
    let items = Parser::new(&tokens).parse()?;
    TypeChecker::new().check(&items)?;
    Compiler::new().compile(&items)
}
