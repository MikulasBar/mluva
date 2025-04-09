

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

    // pub fn is_true(&self) -> Result<bool, InterpreterError> {
    //     match self {
    //         Self::Bool(b) => Ok(*b),
    //         _ => Err(InterpreterError::TypeError),
    //     }
    // }

    // pub fn expect_int(&self) -> u64 {
    //     let Value::Int(int) = self else {
    //         panic!("Expected an integer, but got {:?}", self);
    //     };
    //     *int
    // }

    // pub fn expect_bool(&self) -> bool {
    //     let Value::Bool(bool) = self else {
    //         panic!("Expected an integer, but got {:?}", self);
    //     };
    //     *bool
    // }

    // pub fn expect_float(&self) -> f64 {
    //     let Value::Float(float) = self else {
    //         panic!("Expected an integer, but got {:?}", self);
    //     };
    //     *float
    // }
}

// Operators
// I don't use overloaded operators cause I don't like them and it could be missleading
impl Value {
    pub fn equal(&self, rhs: Self) -> Result<bool, InterpreterError> {
        Ok(*self == rhs)
    }

    pub fn not_equal(&self, rhs: Self) -> Result<bool, InterpreterError> {
        Ok(*self != rhs)
    }

    pub fn add_assign(&mut self, rhs: Self) -> Result<(), InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => *a += b,
            (Self::Float(a), Self::Float(b)) => *a += b,
            _ => return Err(InterpreterError::Unknown)
        }

        Ok(())
    }

    pub fn mul_assign(&mut self, rhs: Self) -> Result<(), InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => *a *= b,
            (Self::Float(a), Self::Float(b)) => *a *= b,
            _ => return Err(InterpreterError::Unknown)
        }

        Ok(())
    }

    pub fn sub_assign(&mut self, rhs: Self) -> Result<(), InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => *a -= b,
            (Self::Float(a), Self::Float(b)) => *a -= b,
            _ => return Err(InterpreterError::Unknown)
        }

        Ok(())
    }

    pub fn div_assign(&mut self, rhs: Self) -> Result<(), InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                *a /= b
            },
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                *a /= b
            },
            _ => return Err(InterpreterError::Unknown)
        }

        Ok(())
    }

    pub fn modulo_assign(&mut self, rhs: Self) -> Result<(), InterpreterError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                *a %= b
            },
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                *a %= b
            },
            _ => return Err(InterpreterError::Unknown)
        }

        Ok(())
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