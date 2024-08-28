use super::TypedExpr;
use crate::{data_type::DataType, value::Value};
use crate::scope::MemoryScope;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Eq,
}

impl BinOp {
    pub fn apply(&self, lhs: &TypedExpr, rhs: &TypedExpr, scope: &MemoryScope) -> Value {
        let lhs = lhs.eval(scope);
        let rhs = rhs.eval(scope);

        match self {
            Self::Add => {
                let lhs = lhs.expect_num();
                let rhs = rhs.expect_num();
                Value::Num(lhs + rhs)
            },

            Self::Sub => {
                let lhs = lhs.expect_num();
                let rhs = rhs.expect_num();
                Value::Num(lhs - rhs)
            },

            Self::Eq => {
                Value::Bool(lhs == rhs)
            },

            _ => todo!()
        }
    }
}