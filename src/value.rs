

use std::fmt::Display;

use crate::expect_pat;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(u64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn expect_int(&self) -> u64 {
        expect_pat!(Value::Int(num) in VAL self);
        *num
    }

    pub fn expect_bool(&self) -> bool {
        expect_pat!(Value::Bool(bool) in VAL self);
        *bool
    }

    pub fn expect_float(&self) -> f64 {
        expect_pat!(Value::Float(num) in VAL self);
        *num
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
            Self::Int(num) => write!(f, "{}", num),
            Self::Float(num) => write!(f, "{}", num),
            Self::Bool(bool) => write!(f, "{}", bool),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}