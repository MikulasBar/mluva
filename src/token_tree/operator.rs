use super::Expr;
use crate::errors::InterpreterError;
use crate::scope::MemoryScope;
use crate::value::Value;

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
    pub fn apply(&self, lhs: &Expr, rhs: &Expr, scope: &MemoryScope) -> Result<Value, InterpreterError> {
        let lhs = lhs.eval(scope)?;
        let rhs = rhs.eval(scope)?;

        let result = match self {
            Self::Add => apply_num_op(lhs, rhs, |l, r| l + r),
            Self::Sub => apply_num_op(lhs, rhs, |l, r| l - r),
            Self::Mul => apply_num_op(lhs, rhs, |l, r| l * r),
            Self::Div => {
                if rhs == Value::Int(0) {
                    return Err(InterpreterError::ValueError);
                }
                apply_num_op(lhs, rhs, |l, r| l / r)
            },
            Self::Modulo => apply_num_op(lhs, rhs, |l, r| l % r),
            Self::Eq => Value::Bool(lhs == rhs),
            Self::Neq => Value::Bool(lhs != rhs),
        };

        Ok(result)
    }
}

fn apply_num_op<F>(lhs: Value, rhs: Value, op: F) -> Value
where
    F: FnOnce(u64, u64) -> u64,
{
    let lhs = lhs.expect_int();
    let rhs = rhs.expect_int();
    Value::Int(op(lhs, rhs))
}
