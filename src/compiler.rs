use std::collections::HashMap;

use crate::{function_table::FunctionTable, instruction::Instruction, token_tree::{BinOp, Expr, Stmt}};


pub struct Compiler<'a> {
    instructions: Vec<Instruction>,
    locals: HashMap<String, usize>,
    next_slot: usize,
    function_table: &'a FunctionTable,
}

impl<'a> Compiler<'a> {
    pub fn new(function_table: &'a FunctionTable) -> Self {
        Self {
            instructions: vec![],
            locals: HashMap::new(),
            next_slot: 0,
            function_table,
        }
    }

    fn get_slot(&mut self, name: &str) -> usize {
        *self.locals.entry(name.to_string()).or_insert_with(|| {
            let slot = self.next_slot;
            self.next_slot += 1;
            slot
        })
    }

    // Returns compiled instructions and number of local slots used
    pub fn compile(mut self, stmts: &[Stmt]) -> (Vec<Instruction>, usize) {
        for stmt in stmts {
            self.compile_stmt(stmt);
        }
    
        (self.instructions, self.next_slot)
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDeclare(_, name, expr) => {
                self.compile_expr(expr);
                let slot = self.get_slot(name);
                self.instructions.push(Instruction::Store(slot));
            },

            Stmt::Expr(expr) => {
                self.compile_expr(expr);
                self.instructions.push(Instruction::Pop);
            },

            _ => todo!(),
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(v) => {
                self.instructions.push(Instruction::Push(v.clone()));
            },
            
            Expr::Var(name) => {
                let slot = self.get_slot(name);
                self.instructions.push(Instruction::Load(slot));
            },

            Expr::BinOp(op, lhs, rhs) => {
                self.compile_expr(lhs);
                self.compile_expr(rhs);
                let op_instruction = bin_op_to_instruction(op);
                self.instructions.push(op_instruction);
            },

            Expr::FuncCall(name, args) => {
                for arg in args {
                    self.compile_expr(arg);
                }

                let Some(slot) = self.function_table.get_slot(name) else {
                    panic!("Function {} not found", name);
                };

                self.instructions.push(Instruction::Call {
                    slot,
                    arg_count: args.len(),
                });
            }
        }
    }
}






fn bin_op_to_instruction(op: &BinOp) -> Instruction {
    match op {
        BinOp::Add => Instruction::Add,
        BinOp::Sub => Instruction::Sub,
        BinOp::Mul => Instruction::Mul,
        BinOp::Div => Instruction::Div,
        BinOp::Modulo => Instruction::Modulo,
        BinOp::Equal => Instruction::Equal,
        BinOp::NotEqual => Instruction::NotEqual,
    }
}