use std::collections::HashMap;

use crate::engine::ExternalFunction;
use crate::errors::InterpreterError;
use crate::token_tree::*;
use crate::scope::MemoryScope;
use crate::value::Value;

type FunctionMap = HashMap<String, Box<dyn ExternalFunction>>;

pub struct Interpreter<'a> {
    scope: MemoryScope,
    functions: &'a FunctionMap,
}

impl<'a> Interpreter<'a> {
    pub fn new(functions: &'a FunctionMap) -> Self {
        Self {
            scope: MemoryScope::new(),
            functions,
        }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), InterpreterError> {
       self.interpret_stmts(stmts)
    }
    
    fn interpret_stmts(&mut self, stmts: &[Stmt]) -> Result<(), InterpreterError> {
       self.scope.enter();
    
        for s in stmts {
            match s {
                Stmt::VarAssign(ident, expr) => {
                    let value = self.eval_expr(&expr)?;
                   self.scope.change(&ident, value);
                },
    
                Stmt::VarDeclare(_, ident, expr) => {
                    let value = self.eval_expr(&expr)?;
                   self.scope.insert_new(ident.clone(), value);
                },
    
                Stmt::Print(expr) => {
                    let value = self.eval_expr(&expr)?;
                    println!("{}", value);
                },
    
                Stmt::If(cond, stmts, else_stmts) => {
                    let cond = self.eval_expr(&cond)?.expect_bool();
                    if cond {
                       self.interpret_stmts(stmts)?;
                    } else if else_stmts.is_some() {
                       self.interpret_stmts(else_stmts.as_ref().unwrap())?;
                    }
                },
    
                Stmt::While(cond, stmts) => {
                    while self.eval_expr(&cond)?.expect_bool() {
                       self.interpret_stmts(stmts)?;
                    }
                },

                Stmt::Expr(expr) => {
                   self.eval_expr(&expr)?;
                },
            }
        }
    
       self.scope.exit();

        Ok(())
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        let result = match expr {
            Expr::Int(num) => num.clone().into(),
            Expr::Float(num) => num.clone().into(),
            Expr::Bool(bool) => bool.clone().into(),
            Expr::Var(ident) => {
                if let Some(value) = self.scope.get(&ident) {
                    value.clone()
                } else {
                    return Err(InterpreterError::UndefinedVariable(ident.clone()));
                }
            }
            Expr::BinOp(op, lhs, rhs) => return self.apply_operator(op, &*lhs, &*rhs),
            Expr::StringLiteral(string) => string.clone().into(),
            Expr::FuncCall(name, args) => {
                let Some(func) = self.functions.get(name) else {
                    return Err(InterpreterError::UndefinedFunction(name.clone()));
                };

                let args = args.iter()
                    .map(|arg| self.eval_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                func.call(args)?
            },
        };

        Ok(result)
    }

    pub fn apply_operator(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Result<Value, InterpreterError> {
        let lhs = self.eval_expr(lhs)?;
        let rhs = self.eval_expr(rhs)?;

        let result = match op {
            BinOp::Add => apply_numeric_op(lhs, rhs, |l, r| l + r, |l, r| l + r),
            BinOp::Sub => apply_numeric_op(lhs, rhs, |l, r| l - r, |l, r| l - r),
            BinOp::Mul => apply_numeric_op(lhs, rhs, |l, r| l * r, |l, r| l * r),
            BinOp::Div => {
                if rhs == Value::Int(0) || rhs == Value::Float(0.0) {
                    return Err(InterpreterError::ValueError);
                }
                apply_numeric_op(lhs, rhs, |l, r| l / r, |l, r| l / r)
            },
            BinOp::Modulo => {
                if rhs == Value::Int(0) || rhs == Value::Float(0.0) {
                    return Err(InterpreterError::ValueError);
                }
                apply_numeric_op(lhs, rhs, |l, r| l % r, |l, r| l % r)
            },
            
            BinOp::Eq => Ok(Value::Bool(lhs == rhs)),
            BinOp::Neq => Ok(Value::Bool(lhs != rhs)),
        };

        result
    }
}


fn apply_numeric_op<FInt, FFloat>(
    lhs: Value,
    rhs: Value,
    int_op: FInt,
    float_op: FFloat,
) -> Result<Value, InterpreterError>
where
    FInt: FnOnce(u64, u64) -> u64,
    FFloat: FnOnce(f64, f64) -> f64,
{
    match (lhs, rhs) {
        (Value::Int(l), Value::Int(r)) => Ok(int_op(l, r).into()),
        (Value::Float(l), Value::Float(r)) => Ok(float_op(l, r).into()),
        _ => Err(InterpreterError::TypeError),
    }
}