use crate::compiler::{data_type::DataType, token::Token};


#[derive(Debug, Clone)]
pub enum CompileError {
    UnexpectedChar(char),
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    WrongType {
        expected: DataType,
        found: DataType,
    },
    WrongNumberOfArguments {
        expected: usize,
        found: usize,
    },
    VariableNotFound(String),
    FunctionNotFound(String),
    VarRedeclaration(String),
}