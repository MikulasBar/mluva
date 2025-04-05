use crate::data_type::DataType;
use super::expr::Expr;



#[derive(Debug, Clone)]
pub enum Stmt {
    VarAssign(String, Expr),
    VarDeclare(Option<DataType>, String, Expr),
    Print(Expr),

    // condition, statements, else statements
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
}
