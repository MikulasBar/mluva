use core::panic;
use std::collections::HashMap;

use crate::errors::CompileError;
use crate::function::{InternalFunctionSigniture, InternalFunctionSource};
use crate::ast::{Ast, BinaryOp, Expr, Stmt, UnaryOp};
use crate::instruction::Instruction;

use crate::module::Module;
use crate::value::Value;

use super::DataType;

pub struct Compiler {
    sources: Vec<InternalFunctionSource>,
    ast: Ast,
}

impl Compiler {
    pub fn new(ast: Ast) -> Self {
        let function_count = ast.function_count() as usize;

        Self {
            sources: Vec::with_capacity(function_count),
            ast,
        }
    }

    pub fn compile(mut self) -> Result<Module, CompileError> {
        for slot in 0..self.ast.function_count() {
            self.compile_function(slot)?;
        }

        let (function_map, function_signitures, ..) = self.ast.deconstruct();
        let main_slot = function_map.get("main").copied();
        let module = Module::new(
            main_slot,
            function_map,
            function_signitures,
            self.sources,
        );

        Ok(module)
    }

    fn compile_function(&mut self, slot: u32) -> Result<(), CompileError> {
        let function_map = self.ast.get_function_map();
        let signiture = self.ast.get_function_signiture_by_slot(slot).unwrap();
        let body = self.ast.get_function_body_by_slot(slot).unwrap();

        let source = FunctionCompiler::new(function_map, body, signiture).compile()?;

        self.sources.push(source);

        Ok(())
    }
}

struct FunctionCompiler<'b> {
    function_map: &'b HashMap<String, u32>,
    body: &'b [Stmt],
    signiture: &'b InternalFunctionSigniture,

    instructions: Vec<Instruction>,
    locals: HashMap<String, usize>,
    next_slot: usize,
}

impl<'b> FunctionCompiler<'b> {
    fn new(
        function_map: &'b HashMap<String, u32>,
        body: &'b [Stmt],
        signiture: &'b InternalFunctionSigniture,
    ) -> Self {
        Self {
            function_map,
            body,
            signiture,
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

    fn update_instruction_at(&mut self, index: usize, inst: Instruction) {
        let len = self.instructions.len();
        if index >= len {
            panic!("Index out of bounds :{}, length: {}", index, len);
        }

        self.instructions[index] = inst;
    }

    fn push(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    fn compile(mut self) -> Result<InternalFunctionSource, CompileError> {
        self.setup_parameters();
        self.compile_stmts(&self.body)?;
        
        // implicit return at the end of Void functions
        if let DataType::Void = self.signiture.return_type {
            self.instructions.push(Instruction::Push(Value::Void));
            self.instructions.push(Instruction::Return);
        }

        Ok(InternalFunctionSource::new(self.locals.len(), self.instructions))
    }

    fn setup_parameters(&mut self) {
        for (name, _) in &self.signiture.params {
            let slot = self.get_slot(&name) as u32;
            self.instructions.push(Instruction::Store { slot });
        }
    }

    fn compile_stmts(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
        for stmt in stmts {
            self.compile_stmt(stmt)?;
        }

        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            // there is no difference between declaration and assignment at this point
            Stmt::VarDeclare(_, name, expr) | Stmt::VarAssign(name, expr) => {
                self.compile_expr(expr)?;
                let slot = self.get_slot(name) as u32;
                self.push(Instruction::Store { slot });
            }

            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
                // We need to pop the expression from stack since we don't use it anywhere.
                self.push(Instruction::Pop);
            }

            Stmt::If(cond, stmts, else_stmts) => {
                self.compile_if_statement(cond, stmts, else_stmts.as_deref())?;
            }

            Stmt::While(cond, stmts) => {
                self.compile_while_statement(cond, stmts)?;
            },

            Stmt::Return(expr) => {
                self.compile_expr(expr)?;
                self.push(Instruction::Return);
            }
        }

        Ok(())
    }

    fn compile_if_statement(&mut self, cond: &Expr, stmts: &[Stmt], else_stmts: Option<&[Stmt]>) -> Result<(), CompileError> {
        // Compile the condition expression
        self.compile_expr(cond)?;

        // Store the index of the jump instruction for the "if" block
        let cond_jump_index = self.instructions.len();
        self.instructions.push(Instruction::JumpIfFalse(0)); // Placeholder instruction

        // Compile the statements in the "if" block
        self.compile_stmts(stmts)?;

        if let Some(else_stmts) = else_stmts {
            // Store the index of the jump instruction for the "else" block
            let if_jump_index = self.instructions.len();
            self.instructions.push(Instruction::Jump(0)); // Placeholder instruction

            // Store the index of else block
            let post_if_index = self.instructions.len();
            // jump from the if condition to the else block
            // we should jump over the whole if-else block, only if block
            self.update_instruction_at(cond_jump_index, Instruction::JumpIfFalse(post_if_index as u32));

            // Compile the statements in the "else" block
            self.compile_stmts(else_stmts)?;

            // Update the jump instruction to skip over the "else" block
            let post_else_index = self.instructions.len();
            self.update_instruction_at(if_jump_index, Instruction::Jump(post_else_index as u32));
        } else {
            // If there is no "else" block, we can just jump over the "if" block
            let post_if_index = self.instructions.len();
            self.update_instruction_at(cond_jump_index, Instruction::JumpIfFalse(post_if_index as u32));
        }

        Ok(())
    }

    fn compile_while_statement(&mut self, cond: &Expr, stmts: &[Stmt]) -> Result<(), CompileError> {
        // Store the index of the start of the "while" block
        // this includes the condition evaluation and check
        // because every iteration we need to check the condition
        let start_index = self.instructions.len();
        self.compile_expr(cond)?;
        // Store the index of the jump instruction so we can update it later
        let cond_jump_index = self.instructions.len();
        self.instructions.push(Instruction::JumpIfFalse(0)); // Placeholder instruction

        // Compile the instructions in the "while" block
        self.compile_stmts(stmts)?;

        // Jump back to the condition check
        self.push(Instruction::Jump(start_index as u32));

        // Index of the end of the "while" block
        let end_index = self.instructions.len();

        // Update the jump instruction for the "while" block to skip over the body and the end jump
        self.update_instruction_at(cond_jump_index, Instruction::JumpIfFalse(end_index as u32));

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Literal(v) => {
                self.instructions.push(Instruction::Push(v.clone()));
            }

            Expr::Var(name) => {
                let slot = self.get_slot(name) as u32;
                self.instructions.push(Instruction::Load { slot });
            }

            Expr::BinaryOp(op, lhs, rhs) => {
                self.compile_expr(lhs)?;
                self.compile_expr(rhs)?;
                let op_instruction = bin_op_to_instruction(op);
                self.instructions.push(op_instruction);
            }

            Expr::UnaryOp(op, expr) => {
                self.compile_expr(expr)?;
                let op_instruction = un_op_to_instruction(op);
                self.instructions.push(op_instruction);
            }

            Expr::FunctionCall(name, args) => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                let Some(call_slot) = self.function_map.get(name).copied() else {
                    panic!("Function {} not found", name);
                };

                self.instructions.push(Instruction::Call {
                    call_slot,
                });
            },

            Expr::ForeignFunctionCall { .. } => todo!(), // TODO
        }

        Ok(())
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
