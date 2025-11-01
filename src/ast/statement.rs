use super::expr::Expr;
use crate::compiler::data_type::DataType;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VarAssign(String, Expr),
    VarDeclare(Option<DataType>, String, Expr),

    // condition, statements, else statements
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    Expr(Expr),
    Return(Expr),
}
