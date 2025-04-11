

use std::fmt::Display;

use super::compiler::data_type::DataType;
use crate::errors::InterpreterError;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Void,
    Int(u64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn get_type(&self) -> DataType {
        match self {
            Self::Void => DataType::Void,
            Self::Int(_) => DataType::Int,
            Self::Float(_) => DataType::Float,
            Self::Bool(_) => DataType::Bool,
            Self::String(_) => DataType::String,
        }
    }

    pub fn is_false(&self) -> Result<bool, InterpreterError> {
        match self {
            Self::Bool(b) => Ok(!b),
            _ => Err(InterpreterError::TypeError),
        }
    }
}

// Operators
// I don't use overloaded operators cause I don't like them and it could be missleading
impl Value {
    pub fn equal(&self, rhs: Self) -> Result<Value, InterpreterError> {
        Ok(Value::Bool(*self == rhs))
    }

    pub fn not_equal(&self, rhs: Self) -> Result<Value, InterpreterError> {
        Ok(Value::Bool(*self != rhs))
    }

    pub fn less(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Bool(*a < b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Bool(*a < b)),
            _ => Err(InterpreterError::TypeError)
        }
    }

    pub fn less_equal(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Bool(*a <= b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Bool(*a <= b)),
            _ => Err(InterpreterError::TypeError)
        }
    }

    pub fn greater(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Bool(*a > b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Bool(*a > b)),
            _ => Err(InterpreterError::TypeError)
        }
    }

    pub fn greater_equal(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Bool(*a >= b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Bool(*a >= b)),
            _ => Err(InterpreterError::TypeError)
        }
    }

    pub fn add(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Int(a + b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Float(a + b)),
            _ => return Err(InterpreterError::TypeError)
        }
    }

    pub fn mul(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Int(a * b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Float(a * b)),
            _ => return Err(InterpreterError::TypeError)
        }
    }

    pub fn sub(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Int(a - b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Float(a - b)),
            _ => return Err(InterpreterError::TypeError)
        }
    }

    pub fn div(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                Ok(Value::Int(a / b))
            },
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                Ok(Value::Float(a / b))
            },
            _ => return Err(InterpreterError::TypeError)
        }
    }

    pub fn modulo(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                Ok(Value::Int(a % b))
            },
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                Ok(Value::Float(a % b))
            },
            _ => return Err(InterpreterError::Unknown)
        }
    }

    pub fn and(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Bool(a), Self::Bool(b)) => Ok(Value::Bool(*a && b)),
            _ => return Err(InterpreterError::TypeError)
        }
    }

    pub fn or(&self, rhs: Self) -> Result<Value, InterpreterError> {
        match (self, rhs) {
            (Self::Bool(a), Self::Bool(b)) => Ok(Value::Bool(*a || b)),
            _ => return Err(InterpreterError::TypeError)
        }
    }

    pub fn not(&self) -> Result<Value, InterpreterError> {
        match self {
            Self::Bool(a) => Ok(Value::Bool(!*a)),
            _ => return Err(InterpreterError::TypeError)
        }
    }
}

mod froms {
    use super::*;

    impl From<u64> for Value {
        fn from(value: u64) -> Self {
            Self::Int(value)
        }
    }

    impl From<f64> for Value {
        fn from(value: f64) -> Self {
            Self::Float(value)
        }
    }

    impl From<bool> for Value {
        fn from(value: bool) -> Self {
            Self::Bool(value)
        }
    }

    impl From<String> for Value {
        fn from(value: String) -> Self {
            Self::String(value)
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Int(num) => write!(f, "{}", num),
            Self::Float(num) => write!(f, "{}", num),
            Self::Bool(bool) => write!(f, "{}", bool),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}