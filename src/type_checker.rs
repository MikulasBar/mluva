use crate::bin_op_pat;
use crate::data_type::DataType;
use crate::token_tree::{Expr, Stmt, BinOp};
use crate::scope::DataTypeScope;
use crate::errors::TypeCheckError;

pub fn type_check(stmts: &[Stmt]) -> Result<(), TypeCheckError> {
    let mut scope = DataTypeScope::new();

    type_check_stmts(&stmts, &mut scope)
}

fn type_check_stmts(stmts: &[Stmt], scope: &mut DataTypeScope) -> Result<(), TypeCheckError> {
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
        Stmt::If(cond, stmts, else_stmts) => {
            let cond = check_expr(cond, scope)?;
            if !cond.is_bool() {
                return Err(TypeCheckError::WrongType{expected: DataType::Bool, found: cond});
            }

            let _ = type_check_stmts(stmts, scope)?;
            if let Some(else_stmts) = else_stmts {
                let _ = type_check_stmts(else_stmts, scope)?;
            }
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

            return type_check_stmts(stmts, scope);
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
        Expr::Float(_) => Ok(DataType::Float),
        Expr::Bool(_) => Ok(DataType::Bool),
        Expr::StringLiteral(_) => Ok(DataType::String),
        Expr::BinOp(op, a, b) => {
            let a = check_expr(a, scope)?;
            let b = check_expr(b, scope)?;
            match op {
                bin_op_pat!(NUMERIC) => {
                    match (a, b) {
                        (DataType::Int, DataType::Int) => Ok(DataType::Int),
                        (DataType::Float, DataType::Float) => Ok(DataType::Float),
                        (DataType::Int | DataType::Float, _) => return Err(TypeCheckError::WrongType{expected: a, found: b}),
                        (_, DataType::Int | DataType::Float) => return Err(TypeCheckError::WrongType{expected: b, found: a}),
                        _ => return Err(TypeCheckError::WrongType{expected: DataType::Int, found: a}),
                    }
                },

                bin_op_pat!(COMPARISON) => {
                    Ok(DataType::Bool)
                },
            }
        },
    }
}
