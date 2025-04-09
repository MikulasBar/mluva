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
    VariableNotFound(String),
    FunctionNotFound(String),
}