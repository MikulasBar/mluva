use std::collections::HashMap;

use crate::token_tree::statement::{Stmt, VarAssign};
use crate::value::Value;

pub type Memory = HashMap<String, Value>;


pub fn interpret(stmts: Vec<Stmt>) {
    let mut mem: Memory = Memory::new();

    for s in stmts {
        match s {
            Stmt::VarAssign(VarAssign{ident, expr}) => {
                let value = expr.eval(&mem).into();
                mem.insert(ident, value);
            },

            Stmt::Print(ident) => {
                let value = mem.get(&ident).unwrap();

                println!("{}", value);
            },
        }
    }
}

