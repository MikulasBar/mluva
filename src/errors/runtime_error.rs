use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    ValueStackUnderflow,
    DivisionByZero,
    Unknown,

    /// this is for any type error
    /// these errors should be handled in the type checker
    /// but I want to have a fallback for any type error
    TypeError,
    FunctionDidNotReturn,
    AssertionFailed,
    Other(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::ValueStackUnderflow => write!(f, "Value stack underflow"),
            RuntimeError::DivisionByZero => write!(f, "Division by zero"),
            RuntimeError::Unknown => write!(f, "Unknown runtime error"),
            RuntimeError::TypeError => write!(f, "Type error"),
            RuntimeError::FunctionDidNotReturn => write!(f, "Function did not return a value"),
            RuntimeError::AssertionFailed => write!(f, "Assertion failed"),
            RuntimeError::Other(msg) => write!(f, "{}", msg),
        }
    }
}
