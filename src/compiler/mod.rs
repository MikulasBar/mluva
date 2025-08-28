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

// pub fn compile_from_str(input: &str) -> Result<ExecutableModule, CompileError> {
//     let tokens = tokenize(input)?;
//     let items = Parser::new(&tokens).parse()?;
//     let (fn_map, _) = TypeChecker::new().check_and_return_definitions(&items)?;
//     let (module, _) = Compiler::new(fn_map).compile(&items)?;
//     Ok(module)
// }

// pub fn compile_from_str_to_bc(input: &str) -> Result<(ExecutableModule, Vec<u8>), CompileError> {
//     let version = 1;
//     let tokens = tokenize(input)?;
//     let items = Parser::new(&tokens).parse()?;
//     let (fn_map, definitions) = TypeChecker::new().check_and_return_definitions(&items)?;
//     let (exec_module, fn_map) = Compiler::new(fn_map).compile(&items)?;
//     // let bytecode = Bytecode::from_executable(version, fn_map, definitions, &exec_module).serialize();

//     Ok((exec_module, vec![]))
// }
