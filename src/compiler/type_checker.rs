use std::collections::HashMap;

use super::data_type::DataType;
use super::data_type_scope::DataTypeScope;
use crate::ast::{BinaryOp, Expr, Item, Stmt, UnaryOp};
use crate::bin_op_pat;
use crate::errors::CompileError;
use crate::function::InternalFunctionDefinition;

pub struct TypeChecker<'a> {
    fn_map: HashMap<String, u32>,
    definitions: Vec<&'a InternalFunctionDefinition>,
}

impl<'a> TypeChecker<'a> {
    pub fn new() -> Self {
        Self {
            fn_map: HashMap::new(),
            definitions: Vec::new(),
        }
    }

    pub fn check(&mut self, items: &'a [Item]) -> Result<(), CompileError> {
        self.load_function_types(items)?;
        self.check_items(items)
    }

    pub fn check_and_return_definitions(
        mut self,
        items: &'a [Item],
    ) -> Result<(HashMap<String, u32>, Vec<&'a InternalFunctionDefinition>), CompileError> {
        self.load_function_types(items)?;
        self.check_items(items)?;
        Ok((self.fn_map, self.definitions))
    }

    fn load_function_types(&mut self, items: &'a [Item]) -> Result<(), CompileError> {
        for item in items {
            match item {
                Item::FunctionDef(def) => {
                    let name = def.name.clone();
                    if self.fn_map.contains_key(&name) {
                        return Err(CompileError::FunctionAlreadyDefined(name));
                    }

                    let slot = self.fn_map.len() as u32;
                    self.definitions.push(def);
                    self.fn_map.insert(name, slot);
                },

                _ => (),
            }
        }

        Ok(())
    }

    fn check_items(&mut self, items: &[Item]) -> Result<(), CompileError> {
        for item in items {
            match item {
                Item::FunctionDef(fn_def) => {
                    let mut scope = DataTypeScope::new();
                    scope.enter(); // argument scope layer

                    for (arg_name, arg_type) in fn_def.params.iter() {
                        scope.insert_new(arg_name.clone(), arg_type.clone())?;
                    }

                    self.check_stmts(&fn_def.body, &mut scope, fn_def.return_type)?;
                },

                Item::Import(_) => (), // TODO: handle this
            }
        }

        Ok(())
    }

    fn check_stmts(
        &self,
        stmts: &[Stmt],
        scope: &mut DataTypeScope,
        return_type: DataType,
    ) -> Result<(), CompileError> {
        scope.enter();

        for s in stmts {
            self.check_stmt(s, scope, return_type)?;
        }

        scope.exit();

        Ok(())
    }

    fn check_stmt(
        &self,
        stmt: &Stmt,
        scope: &mut DataTypeScope,
        return_type: DataType,
    ) -> Result<(), CompileError> {
        match stmt {
            Stmt::If(cond, stmts, else_stmts) => {
                let cond = self.check_expr(cond, scope)?;
                if !cond.is_bool() {
                    return Err(CompileError::WrongType {
                        expected: DataType::Bool,
                        found: cond,
                    });
                }

                self.check_stmts(stmts, scope, return_type)?;
                if let Some(else_stmts) = else_stmts {
                    self.check_stmts(else_stmts, scope, return_type)?;
                }
            }

            Stmt::VarDeclare(data_type, ident, expr) => {
                // if the declaration has explicit type or not
                // check the type if yes
                // if no then do essentialy nothing
                let expr_type = self.check_expr(expr, scope)?;
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

                scope.insert_new(ident.clone(), data_type)?;
            }

            Stmt::VarAssign(ident, expr) => {
                let Some(&data_type) = scope.get(&ident) else {
                    return Err(CompileError::VariableNotFound(ident.clone()));
                };

                let expr_type = self.check_expr(expr, scope)?;

                if expr_type != data_type {
                    return Err(CompileError::WrongType {
                        expected: data_type,
                        found: expr_type,
                    });
                }
            }

            Stmt::While(cond, stmts) => {
                let cond = self.check_expr(cond, scope)?;
                if !cond.is_bool() {
                    return Err(CompileError::WrongType {
                        expected: DataType::Bool,
                        found: cond,
                    });
                }

                return self.check_stmts(stmts, scope, return_type);
            }

            Stmt::Expr(expr) => {
                self.check_expr(expr, scope)?;
            }

            Stmt::Return(expr) => {
                let expr_type = self.check_expr(expr, scope)?;
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

    fn check_expr(&self, expr: &Expr, scope: &mut DataTypeScope) -> Result<DataType, CompileError> {
        match expr {
            Expr::Var(ident) => {
                let Some(data_type) = scope.get(ident) else {
                    return Err(CompileError::VariableNotFound(ident.clone()));
                };

                Ok(data_type.clone())
            }
            Expr::Literal(lit) => Ok(lit.get_type()),
            Expr::FuncCall(name, args) => {
                let Some(func) = self.fn_map.get(name).map(|i| self.definitions[*i as usize])
                else {
                    return Err(CompileError::FunctionNotFound(name.clone()));
                };

                let arg_types: Vec<DataType> = args
                    .iter()
                    .map(|arg| self.check_expr(arg, scope))
                    .collect::<Result<Vec<DataType>, CompileError>>()?;

                // println!("ARG TYPES: {:?}", arg_types);
                func.check_arg_types(&arg_types)?;

                Ok(func.return_type)
            },

            Expr::ForeignFuncCall { .. } => todo!(), // TODO: handle this

            Expr::BinaryOp(op, a, b) => {
                let a = self.check_expr(a, scope)?;
                let b = self.check_expr(b, scope)?;
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
                let a = self.check_expr(a, scope)?;
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
