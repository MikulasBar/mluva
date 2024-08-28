use super::typed_expr::TypedExpr;


pub enum TypedStmt {
    VarAssign(String, TypedExpr),
    VarDeclare(String, TypedExpr),
    Print(TypedExpr),
    If(TypedExpr, Vec<TypedStmt>),
    While(TypedExpr, Vec<TypedStmt>),
}