use crate::errors::InterpreterError;
use crate::token_tree::*;
use crate::scope::MemoryScope;

pub fn interpret(stmts: Vec<Stmt>) -> Result<(), InterpreterError> {
    let mut scope = MemoryScope::new();

    interpret_helper(&stmts, &mut scope)
}

fn interpret_helper(stmts: &Vec<Stmt>, scope: &mut MemoryScope) -> Result<(), InterpreterError> {
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

            Stmt::If(cond, stmts) => {
                let cond = cond.eval(scope)?.expect_bool();
                if cond {
                    interpret_helper(stmts, scope)?;
                }
            },

            Stmt::While(cond, stmts) => {
                while cond.eval(scope)?.expect_bool() {
                    interpret_helper(stmts, scope)?;
                }
            },
        }
    }

    scope.exit();
    Ok(())
}

