use crate::data_type::DataType;
use super::expr::Expr;



#[derive(Debug, Clone)]
pub enum Stmt {
    VarAssign(String, Expr),
    VarDeclare(DataType, String, Expr),
    Print(Expr),
    If(Expr, Vec<Stmt>),
}

impl Stmt {
    // pub fn var_assign(ident: String, expr: Expr) -> Self {
    //     Self::VarAssign(VarAssign::new(ident, expr))
    // }

    // pub fn print(ident: String) -> Self {
    //     Self::Print(ident)
    // }

    // pub fn if_statement(cond: Expr, stmts: Vec<Stmt>) -> Self {
    //     Self::If(IfStatement::new(cond, stmts))
    // }

    // pub fn var_declare(data_type: DataType, ident: String, expr: Expr) -> Self {
    //     Self::VarDeclare(VarDeclare::new(data_type, ident, expr))
    // }
}

// #[derive(Debug, Clone)]
// pub struct VarAssign {
//     pub ident: String,
//     pub expr: Expr,
// }

// impl VarAssign {
//     pub fn new(ident: String, expr: Expr) -> Self {
//         Self {
//             ident,
//             expr,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct IfStatement {
//     pub cond: Expr,
//     pub stmts: Vec<Stmt>,
// }


// impl IfStatement {
//     pub fn new(cond: Expr, stmts: Vec<Stmt>) -> Self {
//         Self {
//             cond,
//             stmts
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct VarDeclare {
//     pub data_type: DataType,
//     pub ident: String,
//     pub expr: Expr
// }

// impl VarDeclare {
//     pub fn new(data_type: DataType, ident: String, expr: Expr) -> Self {
//         Self {
//             data_type,
//             ident,
//             expr,
//         }
//     }
// }