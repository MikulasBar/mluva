

use core::panic;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Void,
    Int(u64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn get_type(&self) -> crate::data_type::DataType {
        match self {
            Self::Void => crate::data_type::DataType::Void,
            Self::Int(_) => crate::data_type::DataType::Int,
            Self::Float(_) => crate::data_type::DataType::Float,
            Self::Bool(_) => crate::data_type::DataType::Bool,
            Self::String(_) => crate::data_type::DataType::String,
        }
    }

    pub fn expect_int(&self) -> u64 {
        let Value::Int(int) = self else {
            panic!("Expected an integer, but got {:?}", self);
        };
        *int
    }

    pub fn expect_bool(&self) -> bool {
        let Value::Bool(bool) = self else {
            panic!("Expected an integer, but got {:?}", self);
        };
        *bool
    }

    pub fn expect_float(&self) -> f64 {
        let Value::Float(float) = self else {
            panic!("Expected an integer, but got {:?}", self);
        };
        *float
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