use crate::bin_op_pat;
use super::data_type::DataType;
use crate::function_table::FunctionTable;
use super::token_tree::{BinaryOp, Expr, Stmt, UnaryOp};
use super::data_type_scope::DataTypeScope;
use crate::errors::CompileError;


pub struct TypeChecker<'a> {
    scope: DataTypeScope,
    function_table: &'a FunctionTable,
}

impl<'a> TypeChecker<'a> {
    pub fn new(function_table: &'a FunctionTable) -> Self {
        Self {
            scope: DataTypeScope::new(),
            function_table,
        }
    }
        
    pub fn check(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
       self.check_stmts(stmts)
    }

    fn check_stmts(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
       self.scope.enter();

        for s in stmts {
           self.check_stmt(s)?;
        }

       self.scope.exit();

        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::If(cond, stmts, else_stmts) => {
                let cond = self.check_expr(cond)?;
                if !cond.is_bool() {
                    return Err(CompileError::WrongType{expected: DataType::Bool, found: cond});
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
                        return Err(CompileError::WrongType{expected: *data_type, found: expr_type});
                    }

                    *data_type
                } else {
                    expr_type
                };

               self.scope.insert_new(ident.clone(), data_type)?;
            },

            Stmt::VarAssign(ident, expr) => {
                let Some(&data_type) = self.scope.get(&ident) else {
                    return Err(CompileError::VariableNotFound(ident.clone()));
                };

                let expr_type = self.check_expr(expr)?;

                if expr_type != data_type {
                    return Err(CompileError::WrongType{expected: data_type, found: expr_type});
                }
            },

            Stmt::While(cond, stmts) => {
                let cond = self.check_expr(cond)?;
                if !cond.is_bool() {
                    return Err(CompileError::WrongType{expected: DataType::Bool, found: cond});
                }

                return self.check_stmts(stmts);
            },

            Stmt::Expr(expr) => {
               self.check_expr(expr)?;
            },
        }

        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<DataType, CompileError> {
        match expr {
            Expr::Var(ident) => {
                let Some(data_type) = self.scope.get(ident) else {
                    return Err(CompileError::VariableNotFound(ident.clone()));
                };

                Ok(data_type.clone())
            },
            Expr::Literal(lit) => Ok(lit.get_type()),
            Expr::FuncCall(name, args) => {
                let Some(func) = self.function_table.get_fn(name.as_str()) else {
                    return Err(CompileError::FunctionNotFound(name.clone()));
                };

                let arg_types: Vec<DataType> = args.iter()
                    .map(|arg| self.check_expr(arg))
                    .collect::<Result<Vec<DataType>, CompileError>>()?;

                func.check_types(&arg_types)?;

                Ok(func.return_type)
            }

            Expr::BinaryOp(op, a, b) => {
                let a = self.check_expr(a)?;
                let b = self.check_expr(b)?;
                match op {
                    bin_op_pat!(NUMERIC) => {
                        match (a, b) {
                            (DataType::Int, DataType::Int) => Ok(DataType::Int),
                            (DataType::Float, DataType::Float) => Ok(DataType::Float),
                            (DataType::Int | DataType::Float, _) => return Err(CompileError::WrongType{expected: a, found: b}),
                            (_, DataType::Int | DataType::Float) => return Err(CompileError::WrongType{expected: b, found: a}),
                            _ => return Err(CompileError::WrongType{expected: DataType::Int, found: a}),
                        }
                    },
                    
                    bin_op_pat!(NUMERIC_COMPARISON) => {
                        match (a, b) {
                            (DataType::Int, DataType::Int) => Ok(DataType::Bool),
                            (DataType::Float, DataType::Float) => Ok(DataType::Bool),
                            (DataType::Int | DataType::Float, _) => return Err(CompileError::WrongType{expected: a, found: b}),
                            (_, DataType::Int | DataType::Float) => return Err(CompileError::WrongType{expected: b, found: a}),
                            _ => return Err(CompileError::WrongType{expected: DataType::Int, found: a}),
                        }
                    },

                    bin_op_pat!(COMPARISON) => {
                        Ok(DataType::Bool)
                    },

                    bin_op_pat!(LOGICAL) => {
                        if a != DataType::Bool {
                            return Err(CompileError::WrongType{expected: DataType::Bool, found: a});
                        }

                        if b != DataType::Bool {
                            return Err(CompileError::WrongType{expected: DataType::Bool, found: b});
                        }

                        Ok(DataType::Bool)
                    },
                }
            },

            Expr::UnaryOp(op, a) => {
                let a = self.check_expr(a)?;
                match op {
                    UnaryOp::Not => {
                        if a != DataType::Bool {
                            return Err(CompileError::WrongType{expected: DataType::Bool, found: a});
                        }

                        Ok(DataType::Bool)
                    },
                }
            },
        }
    }
}