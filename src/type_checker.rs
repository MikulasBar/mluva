use std::collections::HashMap;

use crate::bin_op_pat;
use crate::data_type::DataType;
use crate::external::ExternalFunction;
use crate::token_tree::{Expr, Stmt, BinOp};
use crate::scope::DataTypeScope;
use crate::errors::TypeCheckError;

type FunctionMap = HashMap<&'static str, ExternalFunction>;

pub struct TypeChecker<'a> {
    scope: DataTypeScope,
    functions: &'a FunctionMap,
}

impl<'a> TypeChecker<'a> {
    pub fn new(functions: &'a FunctionMap) -> Self {
        Self {
            scope: DataTypeScope::new(),
            functions,
        }
    }
        
    pub fn check(&mut self, stmts: &[Stmt]) -> Result<(), TypeCheckError> {
       self.check_stmts(stmts)
    }

    fn check_stmts(&mut self, stmts: &[Stmt]) -> Result<(), TypeCheckError> {
       self.scope.enter();

        for s in stmts {
           self.check_stmt(s)?;
        }

       self.scope.exit();

        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeCheckError> {
        match stmt {
            Stmt::If(cond, stmts, else_stmts) => {
                let cond = self.check_expr(cond)?;
                if !cond.is_bool() {
                    return Err(TypeCheckError::WrongType{expected: DataType::Bool, found: cond});
                }

               self.check_stmts(stmts)?;
                if let Some(else_stmts) = else_stmts {
                   self.check_stmts(else_stmts)?;
                }
            },

            Stmt::VarDeclare(data_type, ident, expr) => {
                // if the declaration has explicit type or not
                // check the type if yes
                // if no then do essentialy nothing
                let expr_type = self.check_expr(expr)?;
                let data_type = if let Some(data_type) = data_type {
                    if expr_type != *data_type {
                        return Err(TypeCheckError::WrongType{expected: *data_type, found: expr_type});
                    }

                    *data_type
                } else {
                    expr_type
                };

               self.scope.insert_new(ident.clone(), data_type);
            },

            Stmt::VarAssign(ident, expr) => {
                let Some(&data_type) = self.scope.get(&ident) else {
                    return Err(TypeCheckError::VariableNotFound(ident.clone()));
                };

                let expr_type = self.check_expr(expr)?;

                if expr_type != data_type {
                    return Err(TypeCheckError::WrongType{expected: data_type, found: expr_type});
                }
            },

            Stmt::While(cond, stmts) => {
                let cond = self.check_expr(cond)?;
                if !cond.is_bool() {
                    return Err(TypeCheckError::WrongType{expected: DataType::Bool, found: cond});
                }

                return self.check_stmts(stmts);
            },

            Stmt::Expr(expr) => {
               self.check_expr(expr)?;
            },
        }

        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<DataType, TypeCheckError> {
        match expr {
            Expr::Var(ident) => {
                let Some(data_type) = self.scope.get(ident) else {
                    return Err(TypeCheckError::VariableNotFound(ident.clone()));
                };

                Ok(data_type.clone())
            },
            Expr::Int(_) => Ok(DataType::Int),
            Expr::Float(_) => Ok(DataType::Float),
            Expr::Bool(_) => Ok(DataType::Bool),
            Expr::StringLiteral(_) => Ok(DataType::String),

            Expr::FuncCall(name, args) => {
                let Some(func) = self.functions.get(name.as_str()) else {
                    return Err(TypeCheckError::FunctionNotFound(name.clone()));
                };

                let arg_types: Vec<DataType> = args.iter()
                    .map(|arg| self.check_expr(arg))
                    .collect::<Result<Vec<DataType>, TypeCheckError>>()?;

                func.check_types(&arg_types)?;

                Ok(func.return_type)
            }

            Expr::BinOp(op, a, b) => {
                let a = self.check_expr(a)?;
                let b = self.check_expr(b)?;
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

}
