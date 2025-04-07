//! type and untyped variants are there for this:
//! the untyped variant is used when we are parsing (we dont check the types)
//! the untyped variants then will be type-checked
//! and that will convert them into typed variants



pub mod expr;
pub mod statement;
pub mod operator;


pub use self::{
    expr::Expr,
    statement::Stmt,
    operator::BinOp,
};
