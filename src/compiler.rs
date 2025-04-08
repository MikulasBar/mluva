// use std::collections::HashMap;

// use crate::{instruction::Instruction, token_tree::{BinOp, Expr, Stmt}};


// pub struct Compiler {
//     locals: HashMap<String, usize>,
//     next_slot: usize,
//     instructions: Vec<Instruction>,
// }

// impl Compiler {
//     pub fn new() -> Self {
//         Self {
//             locals: HashMap::new(),
//             next_slot: 0,
//             instructions: vec![],
//         }
//     }

//     pub fn compile(mut self, stmts: &[Stmt]) -> Vec<Instruction> {
//         for stmt in stmts {
//             self.compile_stmt(stmt);
//         }
    
//         self.instructions
//     }

    

//     pub fn compile_stmt(&mut self, stmt: &Stmt) {
        
//     }
// }






// fn compile_expr(expr: &Expr, instructions: &mut Vec<Instruction>) {
//     match expr {
//         Expr::BinOp(op, lhs, rhs) => {
//             compile_expr(lhs, instructions);
//             compile_expr(rhs, instructions);
//             instructions.push(bin_op_to_instruction(op));
//         }
        
//         Expr::Literal(value) => {
//             instructions.push(Instruction::Push(value.clone()));
//         }
        
//         Expr::Var()
//     }
// }


// fn bin_op_to_instruction(op: &BinOp) -> Instruction {
//     match op {
//         BinOp::Add => Instruction::Add,
//         BinOp::Sub => Instruction::Sub,
//         BinOp::Mul => Instruction::Mul,
//         BinOp::Div => Instruction::Div,
//         BinOp::Modulo => Instruction::Modulo,
//         BinOp::Equal => Instruction::Equal,
//         BinOp::NotEqual => Instruction::NotEqual,
//         _ => panic!("Unknown binary operator: {:?}", op),
//     }
// }