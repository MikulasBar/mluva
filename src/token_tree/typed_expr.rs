use crate::data_type;
use crate::{token_tree::Expr, data_type::DataType};
use crate::scope::MemoryScope;
use crate::token_tree::operator::BinOp;
use crate::value::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum TypedExpr {
    Num(u64, DataType),
    Bool(bool, DataType),
    Var(String, DataType),
    BinOp(BinOp, Box<Self>, Box<Self>, DataType),
}


impl TypedExpr {
    pub fn eval(&self, mem: &MemoryScope) -> Value {
        match self {
            &Self::Num(num, _)              => num.into(),
            &Self::Bool(bool, _)            => bool.into(),
            Self::Var(ident, _)             => mem[ident],
            Self::BinOp(op, lhs, rhs, _)    => op.apply(&*lhs, &*rhs, mem),
        }
    }

    pub fn bin_op(op: BinOp, lhs: TypedExpr, rhs: TypedExpr, data_type: DataType) -> Self {
        Self::BinOp(op, Box::new(lhs), Box::new(rhs), data_type)
    }

    pub fn get_type(&self) -> DataType {
        match self {
            Self::Num(_, t)     |
            Self::Bool(_, t)    |
            Self::Var(_, t)     |  
            Self::BinOp(.., t)  => *t,  
        }
    }
}

