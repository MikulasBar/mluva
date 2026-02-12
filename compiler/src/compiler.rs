use core::panic;
use std::collections::HashMap;

use common::ast::{
    BinaryOp, Expr, ExprKind, Pattern, PatternKind, Statement, StatementKind, UnaryOp,
};
use common::function::{FunctionCode, FunctionSigniture, Parameter};
use common::module::{ModuleAST, ModuleCode, ModuleSigniture};
use common::{CompileError, Descriptor, Instruction, Word};

use crate::local_slot::LocalSlot;

pub struct Compiler<'a> {
    ast: &'a ModuleAST,
    dependencies: &'a HashMap<Descriptor, ModuleSigniture>,
    code: ModuleCode,
    string_slots: HashMap<String, u32>,
}

impl<'a> Compiler<'a> {
    pub fn new(ast: &'a ModuleAST, dependencies: &'a HashMap<Descriptor, ModuleSigniture>) -> Self {
        Self {
            ast,
            dependencies,
            code: ModuleCode::empty(),
            string_slots: HashMap::new(),
        }
    }

    pub fn compile(mut self) -> Result<ModuleCode, CompileError> {
        for name in self.ast.function_names() {
            self.compile_function(name)?;
        }

        let Compiler {
            code,
            ..
        } = self;

        Ok(code)
    }

    fn compile_function(&mut self, name: String) -> Result<(), CompileError> {
        let signiture = self
            .ast
            .get_function_sig(&name)
            .expect("Function signiture should exist");

        let body = self.ast.get_function_body(&name).expect("Function body should exist");

        let code = FunctionCompiler::new(
            self.dependencies,
            body,
            signiture,
            &mut self.string_slots,
            &mut self.code,
        )
        .compile()?;

        self.code.add_code(name, code);

        Ok(())
    }
}

struct FunctionCompiler<'b> {
    dependencies: &'b HashMap<Descriptor, ModuleSigniture>,
    body: &'b [Statement],
    signiture: &'b FunctionSigniture,
    string_slots: &'b mut HashMap<String, u32>,
    locals: HashMap<String, LocalSlot>,
    next_local_index: u32,
    code: FunctionCode,
    mod_code: &'b mut ModuleCode,
}

impl<'b> FunctionCompiler<'b> {
    fn new(
        dependencies: &'b HashMap<Descriptor, ModuleSigniture>,
        body: &'b [Statement],
        signiture: &'b FunctionSigniture,
        string_slots: &'b mut HashMap<String, u32>,
        mod_code: &'b mut ModuleCode,
    ) -> Self {
        Self {
            dependencies,
            body,
            signiture,
            locals: HashMap::new(),
            code: FunctionCode::empty(),
            string_slots,
            mod_code,
            next_local_index: 0,
        }
    }

    fn emit(&mut self, instr: Instruction) {
        self.code.emit_instr(instr);
    }

    fn emit_load_local(&mut self, slot: LocalSlot) {
        self.emit(Instruction::LoadLocal { slot: slot.index });
    }

    fn emit_drop(&mut self) {
        self.emit(Instruction::Drop);
    }

    fn get_string_slot(&mut self, string: &str) -> u32 {
        *self
            .string_slots
            .entry(string.to_string())
            .or_insert_with(|| {
                let slot = self.mod_code.add_string(string.to_string());
                slot
            })
    }

    fn get_local_slot(&mut self, name: &str) -> LocalSlot {
        *self.locals.entry(name.to_string()).or_insert_with(|| {
            let i = self.next_local_index;
            self.next_local_index += 1;
            LocalSlot {
                index: i as u32,
            }
        })
    }

    fn update_instruction_at(&mut self, index: usize, instr: Instruction) {
        let len = self.code.len();
        if index >= len {
            panic!("Index out of bounds :{}, length: {}", index, len);
        }

        self.code.set_instr_at(index, instr);
    }

    fn compile(mut self) -> Result<FunctionCode, CompileError> {
        self.setup_parameters();
        self.compile_statements(&self.body)?;

        // implicit return at the end of Void functions
        if self.signiture.return_type.is_void_type() {
            self.emit(Instruction::LoadConst(Word::void()));
            self.emit(Instruction::Return);
        }

        self.code.slot_count = self.next_local_index as usize;

        Ok(self.code)
    }

    fn setup_parameters(&mut self) {
        for Parameter { name, .. } in &self.signiture.params {
            let slot = self.get_local_slot(&name);
            self.emit(Instruction::Store { slot: slot.index });
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
                assignee, value, ..
            }
            | StatementKind::VarAssign { assignee, value } => {
                self.compile_expr(&value)?;
                self.compile_pattern(&assignee)?;
            }

            StatementKind::Expr(expr) => {
                self.compile_expr(&expr)?;
                // We need to pop the expression from stack since we don't use it anywhere.
                self.emit(Instruction::Drop);
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
                if let Some(expr) = expr {
                    self.compile_expr(&expr)?;
                }

                self.emit(Instruction::Return);
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
        let cond_jump_index = self.code.len();
        self.emit(Instruction::JumpIfFalse(0)); // Placeholder instruction

        // Compile the statements in the "if" block
        self.compile_statements(stmts)?;

        if let Some(else_stmts) = else_stmts {
            // Store the index of the jump instruction for the "else" block
            let if_jump_index = self.code.len();
            self.emit(Instruction::Jump(0)); // Placeholder instruction

            // Store the index of else block
            let post_if_index = self.code.len();
            // jump from the if condition to the else block
            // we should jump over the whole if-else block, only if block
            self.update_instruction_at(
                cond_jump_index,
                Instruction::JumpIfFalse(post_if_index as u32),
            );

            // Compile the statements in the "else" block
            self.compile_statements(else_stmts)?;

            // Update the jump instruction to skip over the "else" block
            let post_else_index = self.code.len();
            self.update_instruction_at(if_jump_index, Instruction::Jump(post_else_index as u32));
        } else {
            // If there is no "else" block, we can just jump over the "if" block
            let post_if_index = self.code.len();
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
        let start_index = self.code.len();
        self.compile_expr(cond)?;
        // Store the index of the jump instruction so we can update it later
        let cond_jump_index = self.code.len();
        self.emit(Instruction::JumpIfFalse(0)); // Placeholder instruction

        // Compile the instructions in the "while" block
        self.compile_statements(stmts)?;

        // Jump back to the condition check
        self.emit(Instruction::Jump(start_index as u32));

        // Index of the end of the "while" block
        let end_index = self.code.len();

        // Update the jump instruction for the "while" block to skip over the body and the end jump
        self.update_instruction_at(cond_jump_index, Instruction::JumpIfFalse(end_index as u32));

        Ok(())
    }

    fn compile_pattern(&mut self, pattern: &Pattern) -> Result<(), CompileError> {
        match &pattern.kind {
            PatternKind::Variable(var) => {
                let slot = self.get_local_slot(&var);
                self.emit(Instruction::Store { slot: slot.index });
            }
        }

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match &expr.kind {
            ExprKind::I32Literal(val) => {
                self.emit(Instruction::LoadConst(Word::from_i32(*val)));
            }

            ExprKind::F32Literal(val) => {
                self.emit(Instruction::LoadConst(Word::from_f32(*val)));
            }

            ExprKind::BoolLiteral(val) => {
                self.emit(Instruction::LoadConst(Word::from_bool(*val)));
            }

            ExprKind::StringLiteral(_) => todo!(),
            ExprKind::ArrayLiteral(_) => todo!()

            ExprKind::Var(name) => {
                let slot = self.get_local_slot(name, None);
                self.emit(Instruction::LoadLocal { slot: slot.index });
            }

            ExprKind::BinaryOp(op, lhs, rhs) => {
                self.compile_expr(lhs)?;
                self.compile_expr(rhs)?;
                let op_instruction = bin_op_to_instruction(op, expr.ty.as_ref().unwrap().id);
                self.emit(op_instruction);
            }

            ExprKind::UnaryOp(op, expr) => {
                self.compile_expr(expr)?;
                let op_instruction = un_op_to_instruction(op, expr.ty.as_ref().unwrap().id);
                self.emit(op_instruction);
            }

            ExprKind::FunctionCall { function, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                self.emit(Instruction::FunctionCall());
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
                    .and_then(|module| module.get_fn_slot(func_name))
                else {
                    return Err(CompileError::unknown_foreign_function_at(
                        module_name,
                        func_name,
                        expr.span,
                    ));
                };

                self.string_slots
                    .insert(module_name.clone(), self.next_local_index);

                self.emit(Instruction::ForeignCall {
                    module_name_slot: self.next_local_index,
                    call_slot,
                });
                self.next_local_index += 1;
            }
        }

        Ok(())
    }
}

fn bin_op_to_instruction(op: &BinaryOp, result_type: Descriptor) -> Instruction {
    match op {
        BinaryOp::Add if result_type.is_i32_type() => Instruction::I32Add,
        BinaryOp::Sub if result_type.is_i32_type() => Instruction::I32Sub,
        BinaryOp::Mul if result_type.is_i32_type() => Instruction::I32Mul,
        BinaryOp::Div if result_type.is_i32_type() => Instruction::I32Div,
        BinaryOp::Modulo if result_type.is_i32_type() => Instruction::I32Mod,
        BinaryOp::Less if result_type.is_i32_type() => Instruction::I32Less,
        BinaryOp::LessEqual if result_type.is_i32_type() => Instruction::I32LessEqual,
        BinaryOp::Greater if result_type.is_i32_type() => Instruction::I32Greater,
        BinaryOp::GreaterEqual if result_type.is_i32_type() => Instruction::I32GreaterEqual,

        BinaryOp::Add if result_type.is_f32_type() => Instruction::F32Add,
        BinaryOp::Sub if result_type.is_f32_type() => Instruction::F32Sub,
        BinaryOp::Mul if result_type.is_f32_type() => Instruction::F32Mul,
        BinaryOp::Div if result_type.is_f32_type() => Instruction::F32Div,
        BinaryOp::Modulo if result_type.is_f32_type() => Instruction::F32Mod,
        BinaryOp::Less if result_type.is_f32_type() => Instruction::F32Less,
        BinaryOp::LessEqual if result_type.is_f32_type() => Instruction::F32LessEqual,
        BinaryOp::Greater if result_type.is_f32_type() => Instruction::F32Greater,
        BinaryOp::GreaterEqual if result_type.is_f32_type() => Instruction::F32GreaterEqual,

        BinaryOp::And if result_type.is_bool_type() => Instruction::BoolAnd,
        BinaryOp::Or if result_type.is_bool_type() => Instruction::BoolOr,

        BinaryOp::Equal => Instruction::WordEqual,
        BinaryOp::NotEqual => Instruction::WordNotEqual,

        _ => panic!("Unsupported binary operation"),
    }
}

fn un_op_to_instruction(op: &UnaryOp, result_type: Descriptor) -> Instruction {
    match op {
        UnaryOp::Negate if result_type.is_i32_type() => Instruction::I32Negate,
        UnaryOp::Negate if result_type.is_i32_type() => Instruction::F32Negate,
        UnaryOp::Not if result_type.is_bool_type() => Instruction::BoolNot,
        _ => panic!("Unsupported unary operation"),
    }
}
