use super::expr::Expr;



#[derive(Debug, Clone)]
pub enum Stmt {
    VarAssign(VarAssign),
    Print(String),
}

impl Stmt {
    pub fn var_assign(ident: String, expr: Expr) -> Self {
        Self::VarAssign(VarAssign::new(ident, expr))
    }
    pub fn print(ident: String) -> Self {
        Self::Print(ident)
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