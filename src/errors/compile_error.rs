use crate::compiler::{data_type::DataType, token::Token};

#[derive(Debug, Clone)]
pub enum CompileError {
    UnexpectedChar(char),
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    WrongType { expected: DataType, found: DataType },
    WrongNumberOfArguments { expected: usize, found: usize },
    VariableNotFound(String),
    FunctionNotFound(String),
    FunctionAlreadyDefined(String),
    VarRedeclaration(String),
    ModuleNotFound(String),
    UnknownForeignFunction { module: String, name: String },
    ReservedFunctionName(String),
}

impl ToString for CompileError {
    fn to_string(&self) -> String {
        match self {
            CompileError::UnexpectedChar(c) => format!("Unexpected character: '{}'", c),
            CompileError::UnexpectedToken(t) => format!("Unexpected token: {:?}", t),
            CompileError::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
            CompileError::WrongType { expected, found } => {
                format!("Type mismatch: expected {:?}, found {:?}", expected, found)
            }
            CompileError::WrongNumberOfArguments { expected, found } => {
                format!(
                    "Wrong number of arguments: expected {}, found {}",
                    expected, found
                )
            }
            CompileError::VariableNotFound(name) => format!("Variable not found: {}", name),
            CompileError::FunctionNotFound(name) => format!("Function not found: {}", name),
            CompileError::FunctionAlreadyDefined(name) => {
                format!("Function already defined: {}", name)
            }
            CompileError::VarRedeclaration(name) => format!("Variable redeclaration: {}", name),
            CompileError::ModuleNotFound(name) => format!("Module not found: {}", name),
            CompileError::UnknownForeignFunction { module, name } => {
                format!("Unknown foreign function '{}' in module '{}'", name, module)
            }
            CompileError::ReservedFunctionName(name) => {
                format!("'{}' is a reserved function name", name)
            }
        }
    }
}
