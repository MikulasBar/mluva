use crate::token_tree::{Stmt, TypedStmt};
use crate::scope::DataTypeScope;


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
            if !cond.get_type(scope).is_bool() {
                panic!()
            }

            let stmts = type_check_helper(stmts, scope);
            TypedStmt::If(cond.to_typed(scope), stmts)
        },

        Stmt::VarDeclare(data_type, ident, expr) => {
            // if the declration has explicit type or not
            // check the type if yes
            // if no then do essentialy nothing
            let data_type = if let Some(data_type) = data_type {
                if expr.get_type(scope) != data_type {
                    panic!()
                }

                data_type
            } else {
                expr.get_type(scope)
            };

            let expr = expr.to_typed(scope);
            scope.insert_new(ident.clone(), data_type);
            TypedStmt::VarDeclare(ident, expr)
        },

        Stmt::VarAssign(ident, expr) => {
            let data_type = scope.get(&ident).unwrap();

            if expr.get_type(scope) != *data_type {
                panic!()
            }
            
            TypedStmt::VarAssign(ident, expr.to_typed(scope))
        },

        Stmt::Print(expr) => {
            TypedStmt::Print(expr.to_typed(scope))
        },

        Stmt::While(cond, stmts) => {
            if !cond.get_type(scope).is_bool() {
                panic!()
            }

            let stmts = type_check_helper(stmts, scope);
            TypedStmt::While(cond.to_typed(scope), stmts)
        }
    }
}
