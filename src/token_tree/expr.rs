use crate::token_tree::operator::BinOp;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(u64),
    Float(f64),
    Bool(bool),
    Var(String),
    BinOp(BinOp, Box<Self>, Box<Self>),
    StringLiteral(String),
    FuncCall(String, Vec<Expr>),
}

impl Expr {
    pub fn new_bin_op(op: BinOp, lhs: Self, rhs: Self) -> Self {
        Self::BinOp(op, Box::new(lhs), Box::new(rhs))
    }
}
