use super::TypedExpr;
use crate::value::Value;
use crate::scope::MemoryScope;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Eq,
    Neq,
}

impl BinOp {
    pub fn apply(&self, lhs: &TypedExpr, rhs: &TypedExpr, scope: &MemoryScope) -> Value {
        let lhs = lhs.eval(scope);
        let rhs = rhs.eval(scope);

        match self {
            Self::Add       => apply_num_op(lhs, rhs, |l, r| l + r),
            Self::Sub       => apply_num_op(lhs, rhs, |l, r| l - r),
            Self::Mul       => apply_num_op(lhs, rhs, |l, r| l * r),
            Self::Div       => apply_num_op(lhs, rhs, |l, r| l / r),
            Self::Modulo    => apply_num_op(lhs, rhs, |l, r| l % r),
            Self::Eq        => Value::Bool(lhs == rhs),
            Self::Neq       => Value::Bool(lhs != rhs),
        }
    }
}

fn apply_num_op<F>(lhs: Value, rhs: Value, op: F) -> Value
where
    F: FnOnce(u64, u64) -> u64
{
    let lhs = lhs.expect_num();
    let rhs = rhs.expect_num();
    Value::Num( op(lhs, rhs) )
}