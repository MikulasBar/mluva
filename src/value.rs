

use std::fmt::Display;

use crate::expect_pat;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Num(u64),
    Bool(bool),
}

impl Value {
    pub fn expect_num(&self) -> u64 {
        expect_pat!(Value::Num(num) in VAL self);
        *num
    }

    pub fn expect_bool(&self) -> bool {
        expect_pat!(Value::Bool(bool) in VAL self);
        *bool
    }
}


mod froms {
    use super::*;

    impl From<u64> for Value {
        fn from(value: u64) -> Self {
            Self::Num(value)
        }
    }

    impl From<bool> for Value {
        fn from(value: bool) -> Self {
            Self::Bool(value)
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(num)      => write!(f, "{}", num),
            Self::Bool(bool)    => write!(f, "{}", bool),
        }
    }
}