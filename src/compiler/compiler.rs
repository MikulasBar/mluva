use core::panic;
use std::collections::HashMap;

use crate::ast::{
    Ast, BinaryOp, Expr, ExprKind, SpannedFunctionSigniture, SpannedParameter, Statement,
    StatementKind, UnaryOp,
};
use crate::errors::CompileError;
use crate::function::FunctionSource;
use crate::instruction::Instruction;

use crate::module::Module;
use crate::value::Value;

use crate::data_type::DataType;

pub struct Compiler<'a> {
    sources: Vec<FunctionSource>,
    ast: Ast,
    dependencies: &'a HashMap<String, Module>,
}

impl<'a> Compiler<'a> {
    pub fn new(ast: Ast, dependencies: &'a HashMap<String, Module>) -> Self {
        let function_count = ast.function_count() as usize;

        Self {
            sources: Vec::with_capacity(function_count),
            ast,
            dependencies,
        }
    }

    pub fn compile(mut self) -> Result<Module, CompileError> {
        for slot in 0..self.ast.function_count() {
            self.compile_function(slot)?;
        }

        let (function_map, spanned_function_signitures, ..) = self.ast.deconstruct();
        let function_signitures = spanned_function_signitures
            .into_iter()
            .map(Into::into)
            .collect();

        let main_slot = function_map.get("main").copied();
        let module = Module::new(main_slot, function_map, function_signitures, self.sources);

        Ok(module)
    }

    fn compile_function(&mut self, slot: u32) -> Result<(), CompileError> {
        let function_map = self.ast.get_function_map();
        let signiture = self.ast.get_function_signiture_by_slot(slot).unwrap();
        let body = self.ast.get_function_body_by_slot(slot).unwrap();

        let source =
            FunctionCompiler::new(self.dependencies, function_map, body, signiture).compile()?;

        self.sources.push(source);

        Ok(())
    }
}

struct FunctionCompiler<'b> {
    dependencies: &'b HashMap<String, Module>,
    function_map: &'b HashMap<String, u32>,
    body: &'b [Statement],
    signiture: &'b SpannedFunctionSigniture,

    instructions: Vec<Instruction>,
    locals: HashMap<String, usize>,
    next_slot: usize,
}

impl<'b> FunctionCompiler<'b> {
    fn new(
        dependencies: &'b HashMap<String, Module>,
        function_map: &'b HashMap<String, u32>,
        body: &'b [Statement],
        signiture: &'b SpannedFunctionSigniture,
    ) -> Self {
        Self {
            dependencies,
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

    fn compile(mut self) -> Result<FunctionSource, CompileError> {
        self.setup_parameters();
        self.compile_statements(&self.body)?;

        // implicit return at the end of Void functions
        if let DataType::Void = self.signiture.return_type {
            self.instructions.push(Instruction::Push(Value::Void));
            self.instructions.push(Instruction::Return);
        }

        Ok(FunctionSource::new(self.locals.len(), self.instructions))
    }

    fn setup_parameters(&mut self) {
        for SpannedParameter { name, .. } in &self.signiture.params {
            let slot = self.get_slot(&name) as u32;
            self.instructions.push(Instruction::Store { slot });
        }
    }

    fn compile_statements(&mut self, statements: &[Statement]) -> Result<(), CompileError> {
        for statement in statements {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), CompileError> {
        match &statement.kind {
            // there is no difference between declaration and assignment at this point
            StatementKind::VarDeclare {
                variable, value, ..
            }
            | StatementKind::VarAssign { variable, value } => {
                self.compile_expr(&value)?;
                let slot = self.get_slot(&variable) as u32;
                self.push(Instruction::Store { slot });
            }

            StatementKind::Expr(expr) => {
                self.compile_expr(&expr)?;
                // We need to pop the expression from stack since we don't use it anywhere.
                self.push(Instruction::Pop);
            }

            StatementKind::If {
                condition,
                if_block,
                else_block,
            } => {
                self.compile_if_statement(&condition, &if_block, else_block.as_deref())?;
            }

            StatementKind::While { condition, block } => {
                self.compile_while_statement(&condition, &block)?;
            }

            StatementKind::Return(expr) => {
                self.compile_expr(&expr)?;
                self.push(Instruction::Return);
            }
        }

        Ok(())
    }

    fn compile_if_statement(
        &mut self,
        cond: &Expr,
        stmts: &[Statement],
        else_stmts: Option<&[Statement]>,
    ) -> Result<(), CompileError> {
        // Compile the condition expression
        self.compile_expr(cond)?;

        // Store the index of the jump instruction for the "if" block
        let cond_jump_index = self.instructions.len();
        self.instructions.push(Instruction::JumpIfFalse(0)); // Placeholder instruction

        // Compile the statements in the "if" block
        self.compile_statements(stmts)?;

        if let Some(else_stmts) = else_stmts {
            // Store the index of the jump instruction for the "else" block
            let if_jump_index = self.instructions.len();
            self.instructions.push(Instruction::Jump(0)); // Placeholder instruction

            // Store the index of else block
            let post_if_index = self.instructions.len();
            // jump from the if condition to the else block
            // we should jump over the whole if-else block, only if block
            self.update_instruction_at(
                cond_jump_index,
                Instruction::JumpIfFalse(post_if_index as u32),
            );

            // Compile the statements in the "else" block
            self.compile_statements(else_stmts)?;

            // Update the jump instruction to skip over the "else" block
            let post_else_index = self.instructions.len();
            self.update_instruction_at(if_jump_index, Instruction::Jump(post_else_index as u32));
        } else {
            // If there is no "else" block, we can just jump over the "if" block
            let post_if_index = self.instructions.len();
            self.update_instruction_at(
                cond_jump_index,
                Instruction::JumpIfFalse(post_if_index as u32),
            );
        }

        Ok(())
    }

    fn compile_while_statement(
        &mut self,
        cond: &Expr,
        stmts: &[Statement],
    ) -> Result<(), CompileError> {
        // Store the index of the start of the "while" block
        // this includes the condition evaluation and check
        // because every iteration we need to check the condition
        let start_index = self.instructions.len();
        self.compile_expr(cond)?;
        // Store the index of the jump instruction so we can update it later
        let cond_jump_index = self.instructions.len();
        self.instructions.push(Instruction::JumpIfFalse(0)); // Placeholder instruction

        // Compile the instructions in the "while" block
        self.compile_statements(stmts)?;

        // Jump back to the condition check
        self.push(Instruction::Jump(start_index as u32));

        // Index of the end of the "while" block
        let end_index = self.instructions.len();

        // Update the jump instruction for the "while" block to skip over the body and the end jump
        self.update_instruction_at(cond_jump_index, Instruction::JumpIfFalse(end_index as u32));

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match &expr.kind {
            ExprKind::Literal(v) => {
                self.instructions.push(Instruction::Push(v.clone()));
            }

            ExprKind::Var(name) => {
                let slot = self.get_slot(name) as u32;
                self.instructions.push(Instruction::Load { slot });
            }

            ExprKind::BinaryOp(op, lhs, rhs) => {
                self.compile_expr(lhs)?;
                self.compile_expr(rhs)?;
                let op_instruction = bin_op_to_instruction(op);
                self.instructions.push(op_instruction);
            }

            ExprKind::UnaryOp(op, expr) => {
                self.compile_expr(expr)?;
                let op_instruction = un_op_to_instruction(op);
                self.instructions.push(op_instruction);
            }

            ExprKind::FunctionCall { func_name, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                let Some(call_slot) = self.function_map.get(func_name).copied() else {
                    panic!(
                        "Function {} not found, should be handled in typechecker",
                        func_name
                    );
                };

                self.instructions.push(Instruction::Call { call_slot });
            }

            ExprKind::ForeignFunctionCall {
                module_name,
                func_name,
                args,
            } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                let Some(call_slot) = self
                    .dependencies
                    .get(module_name)
                    .and_then(|module| module.get_slot(func_name))
                else {
                    return Err(CompileError::unknown_foreign_function_at(
                        module_name,
                        func_name,
                        expr.span,
                    ));
                };

                self.instructions.push(Instruction::ForeignCall {
                    module_name: module_name.clone(),
                    call_slot,
                });
            }

            ExprKind::BuiltinFunctionCall { function, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                self.instructions.push(Instruction::BuiltinFunctionCall {
                    function: function.clone(),
                    arg_count: args.len() as u32,
                });
            }

            ExprKind::MethodCall {
                callee,
                method_name,
                args,
            } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                self.compile_expr(callee)?;

                self.instructions.push(Instruction::MethodCall {
                    method_name: method_name.clone(),
                    arg_count: args.len() as u32,
                });
            }
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
