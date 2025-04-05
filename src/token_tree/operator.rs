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
            Self::Add => apply_numeric_op(lhs, rhs, |l, r| l + r, |l, r| l + r),
            Self::Sub => apply_numeric_op(lhs, rhs, |l, r| l - r, |l, r| l - r),
            Self::Mul => apply_numeric_op(lhs, rhs, |l, r| l * r, |l, r| l * r),
            Self::Div => {
                if rhs == Value::Int(0) || rhs == Value::Float(0.0) {
                    return Err(InterpreterError::ValueError);
                }
                apply_numeric_op(lhs, rhs, |l, r| l / r, |l, r| l / r)
            },
            Self::Modulo => {
                if rhs == Value::Int(0) || rhs == Value::Float(0.0) {
                    return Err(InterpreterError::ValueError);
                }
                apply_numeric_op(lhs, rhs, |l, r| l % r, |l, r| l % r)
            },
            
            Self::Eq => Ok(Value::Bool(lhs == rhs)),
            Self::Neq => Ok(Value::Bool(lhs != rhs)),
        };

        result
    }
}

fn apply_numeric_op<FInt, FFloat>(
    lhs: Value,
    rhs: Value,
    int_op: FInt,
    float_op: FFloat,
) -> Result<Value, InterpreterError>
where
    FInt: FnOnce(u64, u64) -> u64,
    FFloat: FnOnce(f64, f64) -> f64,
{
    match (lhs, rhs) {
        (Value::Int(l), Value::Int(r)) => Ok(int_op(l, r).into()),
        (Value::Float(l), Value::Float(r)) => Ok(float_op(l, r).into()),
        _ => Err(InterpreterError::TypeError),
    }
}