use super::TypedExpr;
use crate::{data_type::DataType, value::Value};
use crate::scope::MemoryScope;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinOp {
    Add,
    Eq,
}

impl BinOp {
    pub fn apply(&self, lhs: &TypedExpr, rhs: &TypedExpr, mem: &MemoryScope) -> Value {
        let lhs_t = lhs.get_type();
        let rhs_t = rhs.get_type();

        match (self, lhs_t, rhs_t) {
            (Self::Add, DataType::Num, DataType::Num) => {
                let lhs = lhs.eval(mem).expect_num();
                let rhs = rhs.eval(mem).expect_num();
                Value::Num(lhs + rhs)
            },
            (Self::Eq, _, _) => {
                let lhs = lhs.eval(mem);
                let rhs = rhs.eval(mem);
                Value::Bool(lhs == rhs)
            },
            _ => panic!()
        }
    }
}