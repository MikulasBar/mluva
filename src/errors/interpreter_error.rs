use crate::data_type::DataType;



#[derive(Debug, Clone)]
pub enum InterpreterError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    ValueError,
    TypeError,
    WrongArgumentCount {
        expected: usize,
        found: usize,
    },
    WrongArgumentType {
        expected: DataType,
        found: DataType,
    },
}