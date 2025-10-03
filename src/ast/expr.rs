use crate::value::Value;
use super::{binary_op::BinaryOp, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Var(String),
    BinaryOp(BinaryOp, Box<Self>, Box<Self>),
    UnaryOp(UnaryOp, Box<Self>),
    FunctionCall(String, Vec<Expr>),
    ForeignFunctionCall {
        module_name: String,
        func_name: String,
        args: Vec<Expr>,
    }
}

impl Expr {
    pub fn new_binary_op(op: BinaryOp, lhs: Self, rhs: Self) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_unary_op(op: UnaryOp, expr: Self) -> Self {
        Self::UnaryOp(op, Box::new(expr))
    }
}
