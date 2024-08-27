use super::typed_expr::TypedExpr;


pub enum TypedStmt {
    VarAssign(String, TypedExpr),
    Print(TypedExpr),
    If(TypedExpr, Vec<TypedStmt>)
}