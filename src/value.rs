

use core::panic;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Num(u64),
    Bool(bool),
}

impl Value {
    pub fn expect_num(&self) -> u64 {
        let Value::Num(num) = self else {panic!()};
        *num
    }

    pub fn expect_bool(&self) -> bool {
        let Value::Bool(bool) = self else {panic!()};
        *bool
    } 
}


mod value_froms {
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