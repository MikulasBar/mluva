use crate::data_type::DataType;
use crate::token_tree::Stmt;
use crate::scope::DataTypeScope;
use crate::type_check_error::TypeCheckError;

pub fn type_check(stmts: &[Stmt]) -> Result<(), TypeCheckError> {
    let mut scope = DataTypeScope::new();

    type_check_helper(&stmts, &mut scope)
}

fn type_check_helper(stmts: &[Stmt], scope: &mut DataTypeScope) -> Result<(), TypeCheckError> {
    scope.enter();

    for s in stmts {
        check_stmt(s, scope)?;
    }

    scope.exit();

    Ok(())
}

fn check_stmt(stmt: &Stmt, scope: &mut DataTypeScope) -> Result<(), TypeCheckError> {
    match stmt {
        Stmt::Print(_) => (),
        Stmt::If(cond, stmts) => {
            let cond = cond.get_type(scope);
            if !cond.is_bool() {
                return Err(TypeCheckError::WrongType{expected: DataType::Bool, found: cond});
            }

            return type_check_helper(stmts, scope);
        },

        Stmt::VarDeclare(data_type, ident, expr) => {
            // if the declaration has explicit type or not
            // check the type if yes
            // if no then do essentialy nothing
            let expr_type = expr.get_type(scope);
            let data_type = if let Some(data_type) = data_type {
                if expr_type != *data_type {
                    return Err(TypeCheckError::WrongType{expected: *data_type, found: expr_type});
                }

                *data_type
            } else {
                expr_type
            };

            scope.insert_new(ident.clone(), data_type);
        },

        Stmt::VarAssign(ident, expr) => {
            let data_type = scope.get(&ident).unwrap();

            if expr.get_type(scope) != *data_type {
                return Err(TypeCheckError::WrongType{expected: *data_type, found: expr.get_type(scope)});
            }
        },

        Stmt::While(cond, stmts) => {
            let cond = cond.get_type(scope);
            if !cond.is_bool() {
                return Err(TypeCheckError::WrongType{expected: DataType::Bool, found: cond});
            }

            return type_check_helper(stmts, scope);
        }
    }

    Ok(())
}
