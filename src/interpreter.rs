use crate::errors::InterpreterError;
use crate::token_tree::*;
use crate::scope::MemoryScope;

pub fn interpret(stmts: Vec<Stmt>) -> Result<(), InterpreterError> {
    let mut scope = MemoryScope::new();

    interpret_stmts(&stmts, &mut scope)
}

fn interpret_stmts(stmts: &Vec<Stmt>, scope: &mut MemoryScope) -> Result<(), InterpreterError> {
    scope.enter();

    for s in stmts {
        match s {
            Stmt::VarAssign(ident, expr) => {
                let value = expr.eval(scope)?;
                scope.change(ident, value);
            },

            Stmt::VarDeclare(_, ident, expr) => {
                let value = expr.eval(scope)?;
                scope.insert_new(ident.clone(), value);
            },

            Stmt::Print(expr) => {
                let value = expr.eval(scope)?;
                println!("{}", value);
            },

            Stmt::If(cond, stmts, else_stmts) => {
                let cond = cond.eval(scope)?.expect_bool();
                if cond {
                    interpret_stmts(stmts, scope)?;
                } else if else_stmts.is_some(){
                    interpret_stmts(else_stmts.as_ref().unwrap(), scope)?;
                }
            },

            Stmt::While(cond, stmts) => {
                while cond.eval(scope)?.expect_bool() {
                    interpret_stmts(stmts, scope)?;
                }
            },
        }
    }

    scope.exit();
    Ok(())
}

