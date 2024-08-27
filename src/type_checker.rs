use crate::token_tree::{typed_expr, Stmt, TypedStmt};
use crate::scope::{self, DataTypeScope};


pub fn type_check(stmts: Vec<Stmt>) -> Vec<TypedStmt> {
    let mut scope = DataTypeScope::new();

    type_check_helper(stmts, &mut scope)
}

fn type_check_helper(stmts: Vec<Stmt>, scope: &mut DataTypeScope) -> Vec<TypedStmt> {
    scope.enter();
    
    let stmts = stmts.into_iter()
        .map(|s| {
            check_stmt(s, scope)
        })
        .collect();

    scope.exit();

    stmts
}





fn check_stmt(stmt: Stmt, scope: &mut DataTypeScope) -> TypedStmt {
    match stmt {
        Stmt::If(cond, stmts) => {
            if !cond.is_bool_expr(scope) {
                panic!()
            }

            let stmts = type_check_helper(stmts, scope);
            TypedStmt::If(cond.to_typed(scope), stmts)
        },

        Stmt::VarDeclare(data_type, ident, expr) => {
            if !expr.is_data_type(data_type, scope) {
                panic!()
            }

            scope.insert(ident.clone(), data_type);
            TypedStmt::VarAssign(ident, expr.to_typed(scope))
        },

        Stmt::VarAssign(ident, expr) => {
            let data_type = scope.get(&ident).unwrap();

            if !expr.is_data_type(*data_type, scope) {
                panic!()
            }
            
            TypedStmt::VarAssign(ident, expr.to_typed(scope))
        },

        Stmt::Print(expr) => {
            TypedStmt::Print(expr.to_typed(scope))
        },

        _ => todo!(),
    }
}
