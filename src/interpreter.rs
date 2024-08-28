use std::collections::HashMap;

use crate::token_tree::*;
use crate::value::Value;
use crate::scope::MemoryScope;


pub fn interpret(stmts: Vec<TypedStmt>) {
    let mut scope = MemoryScope::new();

    interpret_helper(&stmts, &mut scope);
}

pub fn interpret_helper(stmts: &Vec<TypedStmt>, scope: &mut MemoryScope) {
    scope.enter();

    for s in stmts {
        match s {
            TypedStmt::VarAssign(ident, expr) => {
                let value = expr.eval(scope);
                scope.change(ident, value);
            },

            TypedStmt::VarDeclare(ident, expr) => {
                let value = expr.eval(scope);
                scope.insert_new(ident.clone(), value);
            },

            TypedStmt::Print(expr) => {
                let value = expr.eval(scope);
                println!("{}", value);
            },

            TypedStmt::If(cond, stmts) => {
                let cond = cond.eval(scope).expect_bool();
                if cond {
                    interpret_helper(stmts, scope);
                }
            },

            TypedStmt::While(cond, stmts) => {
                while cond.eval(scope).expect_bool() {
                    interpret_helper(stmts, scope);
                }
            },
        }
    }

    scope.exit()
}

