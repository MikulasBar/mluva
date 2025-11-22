use std::fmt::Display;

use crate::runtime::Runtime;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    InvalidHeapHandle,
    ValueStackUnderflow,
    DivisionByZero,
    Unknown,

    /// this is for any type error
    /// these errors should be handled in the type checker
    /// but I want to have a fallback for any type error
    TypeError,
    FunctionDidNotReturn,
    AssertionFailed,
    IndexOutOfBounds {
        index: i32,
        size: u32,
    },
    Other(String),
}

impl RuntimeError {
    pub fn index_out_of_bounds(index: i32, size: u32) -> Self {
        RuntimeError::IndexOutOfBounds { index, size }
    }
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
            RuntimeError::IndexOutOfBounds { index, size } => {
                write!(
                    f,
                    "Index out of bounds: the len is {} but the index is {}",
                    size, index
                )
            }
            RuntimeError::InvalidHeapHandle => write!(f, "Invalid heap handle"),
            RuntimeError::Other(msg) => write!(f, "{}", msg),
        }
    }
}
