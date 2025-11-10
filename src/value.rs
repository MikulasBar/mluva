use std::fmt::Display;

use super::data_type::DataType;
use crate::errors::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Void,
    Int(i32),
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

    pub fn is_false(&self) -> Result<bool, RuntimeError> {
        match self {
            Self::Bool(b) => Ok(!b),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn method_call(&self, method_name: &str, args: Vec<Value>) -> Result<Value, RuntimeError> {
        match (self, method_name, args.as_slice()) {
            (Value::String(s), "length", &[]) => Ok(Value::Int(s.len() as i32)),

            _ => Err(RuntimeError::Other(format!(
                "Method '{}' not found for type '{}'",
                method_name,
                self.get_type()
            ))),
        }
    }
}

// Operators
// I don't use overloaded operators cause I don't like them and it could be missleading
impl Value {
    pub fn equal(&self, rhs: Self) -> Result<Value, RuntimeError> {
        Ok(Value::Bool(*self == rhs))
    }

    pub fn not_equal(&self, rhs: Self) -> Result<Value, RuntimeError> {
        Ok(Value::Bool(*self != rhs))
    }

    pub fn less(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Bool(*a < b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Bool(*a < b)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn less_equal(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Bool(*a <= b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Bool(*a <= b)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn greater(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Bool(*a > b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Bool(*a > b)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn greater_equal(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Bool(*a >= b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Bool(*a >= b)),
            _ => Err(RuntimeError::TypeError),
        }
    }

    pub fn add(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Int(a + b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Float(a + b)),
            _ => return Err(RuntimeError::TypeError),
        }
    }

    pub fn mul(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Int(a * b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Float(a * b)),
            _ => return Err(RuntimeError::TypeError),
        }
    }

    pub fn sub(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Value::Int(a - b)),
            (Self::Float(a), Self::Float(b)) => Ok(Value::Float(a - b)),
            _ => return Err(RuntimeError::TypeError),
        }
    }

    pub fn div(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Int(a / b))
            }
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Float(a / b))
            }
            _ => return Err(RuntimeError::TypeError),
        }
    }

    pub fn modulo(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Int(a % b))
            }
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Float(a % b))
            }
            _ => return Err(RuntimeError::Unknown),
        }
    }

    pub fn and(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Bool(a), Self::Bool(b)) => Ok(Value::Bool(*a && b)),
            _ => return Err(RuntimeError::TypeError),
        }
    }

    pub fn or(&self, rhs: Self) -> Result<Value, RuntimeError> {
        match (self, rhs) {
            (Self::Bool(a), Self::Bool(b)) => Ok(Value::Bool(*a || b)),
            _ => return Err(RuntimeError::TypeError),
        }
    }

    pub fn not(&self) -> Result<Value, RuntimeError> {
        match self {
            Self::Bool(a) => Ok(Value::Bool(!*a)),
            _ => return Err(RuntimeError::TypeError),
        }
    }

    pub fn negate(&self) -> Result<Value, RuntimeError> {
        match self {
            Self::Int(a) => Ok(Value::Int(-a)),
            Self::Float(a) => Ok(Value::Float(-a)),
            _ => return Err(RuntimeError::TypeError),
        }
    }
}

mod froms {
    use super::*;

    impl From<i32> for Value {
        fn from(value: i32) -> Self {
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
