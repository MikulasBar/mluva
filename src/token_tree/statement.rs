use super::expr::{Expr, BoolExpr};



#[derive(Debug, Clone)]
pub enum Stmt {
    VarAssign(VarAssign),
    Print(String),
    If(IfStatement),
}

impl Stmt {
    pub fn var_assign(ident: String, expr: Expr) -> Self {
        Self::VarAssign(VarAssign::new(ident, expr))
    }

    pub fn print(ident: String) -> Self {
        Self::Print(ident)
    }

    pub fn if_statement(cond: BoolExpr, stmts: Vec<Stmt>) -> Self {
        Self::If(IfStatement::new(cond, stmts))
    }
}

#[derive(Debug, Clone)]
pub struct VarAssign {
    pub ident: String,
    pub expr: Expr,
}

impl VarAssign {
    pub fn new(ident: String, expr: Expr) -> Self {
        Self {
            ident,
            expr,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub cond: BoolExpr,
    pub stmts: Vec<Stmt>,
}


impl IfStatement {
    pub fn new(cond: BoolExpr, stmts: Vec<Stmt>) -> Self {
        Self {
            cond,
            stmts
        }
    }
}