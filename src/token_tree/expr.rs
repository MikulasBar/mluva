use crate::{token_tree::operator::BinOp, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Var(String),
    BinOp(BinOp, Box<Self>, Box<Self>),
    FuncCall(String, Vec<Expr>),
}

impl Expr {
    pub fn new_bin_op(op: BinOp, lhs: Self, rhs: Self) -> Self {
        Self::BinOp(op, Box::new(lhs), Box::new(rhs))
    }
}
