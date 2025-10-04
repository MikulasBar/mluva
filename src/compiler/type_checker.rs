use super::data_type::DataType;
use super::data_type_scope::DataTypeScope;
use crate::ast::{Ast, BinaryOp, Expr, Stmt, UnaryOp};
use crate::bin_op_pat;
use crate::errors::CompileError;
use crate::module::Module;

pub struct TypeChecker<'a> {
    ast: &'a Ast,
    dependencies: &'a [Module],
    scope: DataTypeScope,
}

impl<'a> TypeChecker<'a> {
    pub fn new(ast: &'a Ast, dependencies: &'a [Module]) -> Self {
        Self {
            ast,
            dependencies,
            scope: DataTypeScope::new(),
        }
    }

    pub fn check(mut self) -> Result<(), CompileError> {
        self.check_functions()
    }

    fn check_functions(&mut self) -> Result<(), CompileError> {
        for slot in 0..self.ast.function_count() {
            self.scope.enter();

            self.ast
                .get_function_signiture_by_slot(slot).unwrap()
                .params.iter()
                .try_for_each(|(name, data_type)| {
                    self.scope.insert_new(name.clone(), data_type.clone())
                })?;

            let statements = self.ast.get_function_body_by_slot(slot).unwrap();
            let return_type = self
                .ast
                .get_function_signiture_by_slot(slot)
                .unwrap()
                .return_type;

            self.check_stmts(statements, return_type)?;

            self.scope.exit();
        }

        Ok(())
    }

    fn check_stmts(
        &mut self,
        stmts: &[Stmt],
        return_type: DataType,
    ) -> Result<(), CompileError> {
        for s in stmts {
            self.check_stmt(s, return_type)?;
        }

        Ok(())
    }

    fn check_stmt(
        &mut self,
        stmt: &Stmt,
        return_type: DataType,
    ) -> Result<(), CompileError> {
        match stmt {
            Stmt::If(cond, stmts, else_stmts) => {
                let cond = self.check_expr(cond)?;
                if !cond.is_bool() {
                    return Err(CompileError::WrongType {
                        expected: DataType::Bool,
                        found: cond,
                    });
                }

                self.check_stmts(stmts, return_type)?;
                if let Some(else_stmts) = else_stmts {
                    self.check_stmts(else_stmts, return_type)?;
                }
            }

            Stmt::VarDeclare(data_type, ident, expr) => {
                // if the declaration has explicit type or not
                // check the type if yes
                // if no then do essentialy nothing
                let expr_type = self.check_expr(expr)?;
                let data_type = if let Some(data_type) = data_type {
                    if expr_type != *data_type {
                        return Err(CompileError::WrongType {
                            expected: *data_type,
                            found: expr_type,
                        });
                    }

                    *data_type
                } else {
                    expr_type
                };

                self.scope.insert_new(ident.clone(), data_type)?;
            }

            Stmt::VarAssign(ident, expr) => {
                let Some(&data_type) = self.scope.get(&ident) else {
                    return Err(CompileError::VariableNotFound(ident.clone()));
                };

                let expr_type = self.check_expr(expr)?;

                if expr_type != data_type {
                    return Err(CompileError::WrongType {
                        expected: data_type,
                        found: expr_type,
                    });
                }
            }

            Stmt::While(cond, stmts) => {
                let cond = self.check_expr(cond)?;
                if !cond.is_bool() {
                    return Err(CompileError::WrongType {
                        expected: DataType::Bool,
                        found: cond,
                    });
                }

                return self.check_stmts(stmts, return_type);
            }

            Stmt::Expr(expr) => {
                self.check_expr(expr)?;
            }

            Stmt::Return(expr) => {
                let expr_type = self.check_expr(expr)?;
                if expr_type != return_type {
                    return Err(CompileError::WrongType {
                        expected: return_type,
                        found: expr_type,
                    });
                }
            }
        }

        Ok(())
    }

    fn check_expr(&self, expr: &Expr) -> Result<DataType, CompileError> {
        match expr {
            Expr::Var(ident) => {
                let Some(data_type) = self.scope.get(ident) else {
                    return Err(CompileError::VariableNotFound(ident.clone()));
                };

                Ok(data_type.clone())
            }
            Expr::Literal(lit) => Ok(lit.get_type()),
            Expr::FunctionCall(name, args) => {
                let Some(signiture) = self.ast.get_function_signiture(name) else {
                    return Err(CompileError::FunctionNotFound(name.clone()));
                };

                let arg_types: Vec<DataType> = args
                    .iter()
                    .map(|arg| self.check_expr(arg))
                    .collect::<Result<Vec<DataType>, CompileError>>()?;

                signiture.check_argument_types(&arg_types)?;

                Ok(signiture.return_type)
            },

            Expr::ForeignFunctionCall { .. } => todo!(), // TODO: handle this

            Expr::BinaryOp(op, a, b) => {
                let a = self.check_expr(a)?;
                let b = self.check_expr(b)?;
                match op {
                    bin_op_pat!(NUMERIC) => match (a, b) {
                        (DataType::Int, DataType::Int) => Ok(DataType::Int),
                        (DataType::Float, DataType::Float) => Ok(DataType::Float),
                        (DataType::Int | DataType::Float, _) => {
                            return Err(CompileError::WrongType {
                                expected: a,
                                found: b,
                            })
                        }
                        (_, DataType::Int | DataType::Float) => {
                            return Err(CompileError::WrongType {
                                expected: b,
                                found: a,
                            })
                        }
                        _ => {
                            return Err(CompileError::WrongType {
                                expected: DataType::Int,
                                found: a,
                            })
                        }
                    },

                    bin_op_pat!(NUMERIC_COMPARISON) => match (a, b) {
                        (DataType::Int, DataType::Int) => Ok(DataType::Bool),
                        (DataType::Float, DataType::Float) => Ok(DataType::Bool),
                        (DataType::Int | DataType::Float, _) => {
                            return Err(CompileError::WrongType {
                                expected: a,
                                found: b,
                            })
                        }
                        (_, DataType::Int | DataType::Float) => {
                            return Err(CompileError::WrongType {
                                expected: b,
                                found: a,
                            })
                        }
                        _ => {
                            return Err(CompileError::WrongType {
                                expected: DataType::Int,
                                found: a,
                            })
                        }
                    },

                    bin_op_pat!(COMPARISON) => Ok(DataType::Bool),

                    bin_op_pat!(LOGICAL) => {
                        if a != DataType::Bool {
                            return Err(CompileError::WrongType {
                                expected: DataType::Bool,
                                found: a,
                            });
                        }

                        if b != DataType::Bool {
                            return Err(CompileError::WrongType {
                                expected: DataType::Bool,
                                found: b,
                            });
                        }

                        Ok(DataType::Bool)
                    }
                }
            }

            Expr::UnaryOp(op, a) => {
                let a = self.check_expr(a)?;
                match op {
                    UnaryOp::Not => {
                        if a != DataType::Bool {
                            return Err(CompileError::WrongType {
                                expected: DataType::Bool,
                                found: a,
                            });
                        }

                        Ok(DataType::Bool)
                    }

                    UnaryOp::Negate => match a {
                        DataType::Int => Ok(DataType::Int),
                        DataType::Float => Ok(DataType::Float),
                        _ => {
                            return Err(CompileError::WrongType {
                                expected: DataType::Int,
                                found: a,
                            })
                        }
                    },
                }
            }
        }
    }
}
