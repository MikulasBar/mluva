use core::panic;
use std::collections::HashMap;

use super::ast::{BinaryOp, Expr, FunctionDef, Item, Stmt, UnaryOp};
use crate::{
    function_source::FunctionSource, function_table::FunctionTable, instruction::{self, Instruction},
    interpreter_source::InterpreterSource,
};

pub struct Compiler<'a> {
    functions: Vec<FunctionSource>,
    function_table: &'a FunctionTable,
    main_slot: Option<usize>,
}

impl<'a> Compiler<'a> {
    pub fn new(function_table: &'a FunctionTable) -> Self {
        Self {
            function_table,
            main_slot: None,
            functions: Vec::new(),
        }
    }

    pub fn compile(mut self, items: &[Item]) -> InterpreterSource {
        self.compile_items(items);
        InterpreterSource::new(self.functions, self.main_slot)
    }

    fn compile_items(&mut self, items: &[Item]) {
        for item in items {
            match item {
                Item::FnDef(fn_def) => {
                    let fn_source = FunctionCompiler::new(self).compile(&fn_def);
                    if fn_def.name == "main" {
                        if self.main_slot.is_some() {
                            panic!("Multiple main functions found");
                        } else {
                            self.main_slot = Some(self.functions.len());
                        }
                    }
                    self.functions.push(fn_source);
                }
            }
        }
    }
}

struct FunctionCompiler<'a, 'b> {
    compiler: &'b mut Compiler<'a>,
    instructions: Vec<Instruction>,
    locals: HashMap<String, usize>,
    next_slot: usize,
}

impl<'a, 'b> FunctionCompiler<'a, 'b> {
    fn new(compiler: &'b mut Compiler<'a>) -> Self {
        Self {
            compiler,
            locals: HashMap::new(),
            instructions: Vec::new(),
            next_slot: 0,
        }
    }

    fn get_slot(&mut self, name: &str) -> usize {
        *self.locals.entry(name.to_string()).or_insert_with(|| {
            let slot = self.next_slot;
            self.next_slot += 1;
            slot
        })
    }

    fn update_at(&mut self, index: usize, inst: Instruction) {
        let len = self.instructions.len();
        if index >= len {
            panic!("Index out of bounds :{}, length: {}", index, len);
        }

        self.instructions[index] = inst;
    }

    fn push(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    fn compile(mut self, fn_def: &FunctionDef) -> FunctionSource {
        self.compile_stmts(&fn_def.body);
        FunctionSource::new(self.locals.len(), self.instructions)
    }

    fn compile_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            // there is no difference between declaration and assignment at this point
            Stmt::VarDeclare(_, name, expr) | Stmt::VarAssign(name, expr) => {
                self.compile_expr(expr);
                let slot = self.get_slot(name);
                self.push(Instruction::Store(slot));
            }

            Stmt::Expr(expr) => {
                self.compile_expr(expr);
                // We need to pop the expression from stack since we don't use it anywhere.
                self.push(Instruction::Pop);
            }

            Stmt::If(cond, stmts, else_stmts) => {
                self.compile_if_statement(cond, stmts, else_stmts.as_deref());
            }

            Stmt::While(cond, stmts) => {
                self.compile_while_statement(cond, stmts);
            },

            Stmt::Return(expr) => {
                self.compile_expr(expr);
                self.push(Instruction::Return);
            }
        }
    }

    fn compile_if_statement(&mut self, cond: &Expr, stmts: &[Stmt], else_stmts: Option<&[Stmt]>) {
        // Compile the condition expression
        self.compile_expr(cond);

        // Store the index of the jump instruction for the "if" block
        let cond_jump_index = self.instructions.len();
        self.instructions.push(Instruction::JumpIfFalse(0)); // Placeholder instruction

        // Compile the statements in the "if" block
        self.compile_stmts(stmts);

        if let Some(else_stmts) = else_stmts {
            // Store the index of the jump instruction for the "else" block
            let if_jump_index = self.instructions.len();
            self.instructions.push(Instruction::Jump(0)); // Placeholder instruction

            // Store the index of else block
            let post_if_index = self.instructions.len();
            // jump from the if condition to the else block
            // we should jump over the whole if-else block, only if block
            self.update_at(cond_jump_index, Instruction::JumpIfFalse(post_if_index));

            // Compile the statements in the "else" block
            self.compile_stmts(else_stmts);

            // Update the jump instruction to skip over the "else" block
            let post_else_index = self.instructions.len();
            self.update_at(if_jump_index, Instruction::Jump(post_else_index));
        } else {
            // If there is no "else" block, we can just jump over the "if" block
            let post_if_index = self.instructions.len();
            self.update_at(cond_jump_index, Instruction::JumpIfFalse(post_if_index));
        }
    }

    fn compile_while_statement(&mut self, cond: &Expr, stmts: &[Stmt]) {
        // Store the index of the start of the "while" block
        // this includes the condition evaluation and check
        // because every iteration we need to check the condition
        let start_index = self.instructions.len();
        self.compile_expr(cond);
        // Store the index of the jump instruction so we can update it later
        let cond_jump_index = self.instructions.len();
        self.instructions.push(Instruction::JumpIfFalse(0)); // Placeholder instruction

        // Compile the instructions in the "while" block
        self.compile_stmts(stmts);

        // Jump back to the condition check
        self.push(Instruction::Jump(start_index));

        // Index of the end of the "while" block
        let end_index = self.instructions.len();

        // Update the jump instruction for the "while" block to skip over the body and the end jump
        self.update_at(cond_jump_index, Instruction::JumpIfFalse(end_index));
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(v) => {
                self.instructions.push(Instruction::Push(v.clone()));
            }

            Expr::Var(name) => {
                let slot = self.get_slot(name);
                self.instructions.push(Instruction::Load(slot));
            }

            Expr::BinaryOp(op, lhs, rhs) => {
                self.compile_expr(lhs);
                self.compile_expr(rhs);
                let op_instruction = bin_op_to_instruction(op);
                self.instructions.push(op_instruction);
            }

            Expr::UnaryOp(op, expr) => {
                self.compile_expr(expr);
                let op_instruction = un_op_to_instruction(op);
                self.instructions.push(op_instruction);
            }

            Expr::FuncCall(name, args) => {
                for arg in args {
                    self.compile_expr(arg);
                }

                let Some(slot) = self.compiler.function_table.get_slot(name) else {
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

fn bin_op_to_instruction(op: &BinaryOp) -> Instruction {
    match op {
        BinaryOp::Add => Instruction::Add,
        BinaryOp::Sub => Instruction::Sub,
        BinaryOp::Mul => Instruction::Mul,
        BinaryOp::Div => Instruction::Div,
        BinaryOp::Modulo => Instruction::Modulo,
        BinaryOp::Equal => Instruction::Equal,
        BinaryOp::NotEqual => Instruction::NotEqual,
        BinaryOp::Less => Instruction::Less,
        BinaryOp::LessEqual => Instruction::LessEqual,
        BinaryOp::Greater => Instruction::Greater,
        BinaryOp::GreaterEqual => Instruction::GreaterEqual,
        BinaryOp::And => Instruction::And,
        BinaryOp::Or => Instruction::Or,
    }
}

fn un_op_to_instruction(op: &UnaryOp) -> Instruction {
    match op {
        UnaryOp::Negate => Instruction::Negate,
        UnaryOp::Not => Instruction::Not,
    }
}
