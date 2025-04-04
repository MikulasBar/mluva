use crate::bin_op_pat;
use crate::data_type::DataType;
use crate::token_tree::{Expr, Stmt, BinOp};
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
        Stmt::Print(e) => {
            check_expr(e, scope)?;
        },
        Stmt::If(cond, stmts) => {
            let cond = check_expr(cond, scope)?;
            if !cond.is_bool() {
                return Err(TypeCheckError::WrongType{expected: DataType::Bool, found: cond});
            }

            return type_check_helper(stmts, scope);
        },

        Stmt::VarDeclare(data_type, ident, expr) => {
            // if the declaration has explicit type or not
            // check the type if yes
            // if no then do essentialy nothing
            let expr_type = check_expr(expr, scope)?;
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
            let Some(&data_type) = scope.get(&ident) else {
                return Err(TypeCheckError::VariableNotFound(ident.clone()));
            };

            let expr_type = check_expr(expr, scope)?;

            if expr_type != data_type {
                return Err(TypeCheckError::WrongType{expected: data_type, found: expr_type});
            }
        },

        Stmt::While(cond, stmts) => {
            let cond = check_expr(cond, scope)?;
            if !cond.is_bool() {
                return Err(TypeCheckError::WrongType{expected: DataType::Bool, found: cond});
            }

            return type_check_helper(stmts, scope);
        }
    }

    Ok(())
}

fn check_expr(expr: &Expr, scope: &mut DataTypeScope) -> Result<DataType, TypeCheckError> {
    match expr {
        Expr::Var(ident) => {
            let Some(data_type) = scope.get(ident) else {
                return Err(TypeCheckError::VariableNotFound(ident.clone()));
            };

            Ok(data_type.clone())
        },
        Expr::Int(_) => Ok(DataType::Int),
        Expr::Bool(_) => Ok(DataType::Bool),
        Expr::StringLiteral(_) => Ok(DataType::String),
        Expr::BinOp(op, a, b) => {
            let a = check_expr(a, scope)?;
            let b = check_expr(b, scope)?;
            match op {
                bin_op_pat!(NUMERIC) => {
                    if a != DataType::Int {
                        return Err(TypeCheckError::WrongType{expected: DataType::Int, found: a});
                    }

                    if b != DataType::Int {
                        return Err(TypeCheckError::WrongType{expected: DataType::Int, found: b});
                    }

                    Ok(DataType::Int)
                },

                bin_op_pat!(COMPARISON) => {
                    Ok(DataType::Bool)
                },
            }
        },
    }
}
