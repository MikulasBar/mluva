use core::panic;
use std::collections::HashMap;

use common::ast::{
    BinaryOp, Expr, ExprKind, Pattern, PatternKind, Statement, StatementKind, UnaryOp,
};
use common::function::{FunctionCode, FunctionSigniture, Parameter};
use common::module::{LCP, LCPEntry, ModuleAST, ModuleCode, ModuleSigniture};
use common::{CompileError, Descriptor, Instruction, Word};

use crate::lcp_key::LCPKey;
use crate::local_slot::LocalSlot;

pub struct Compiler<'a> {
    ast: &'a ModuleAST,
    dependencies: &'a HashMap<Descriptor, ModuleSigniture>,
    code: ModuleCode,
    lcp_slots: HashMap<LCPKey, u32>,
    next_lcp_slot: u32,
}

impl<'a> Compiler<'a> {
    pub fn new(ast: &'a ModuleAST, dependencies: &'a HashMap<Descriptor, ModuleSigniture>) -> Self {
        Self {
            ast,
            dependencies,
            code: ModuleCode::empty(),
            lcp_slots: HashMap::new(),
            next_lcp_slot: 0,
        }
    }

    pub fn compile(mut self) -> Result<ModuleCode, CompileError> {
        let module_path = self.ast.module_descriptor.to_string();

        for name in self.ast.function_names() {
            self.compile_function(&module_path, name)?;
        }

        let Compiler { code, .. } = self;

        Ok(code)
    }

    fn compile_function(&mut self, module_path: &str, name: String) -> Result<(), CompileError> {
        let signiture = self
            .ast
            .get_function_sig(&name)
            .expect("Function signiture should exist");

        let body = self
            .ast
            .get_function_body(&name)
            .expect("Function body should exist");

        let code = FunctionCompiler::new(
            self.dependencies,
            body,
            signiture,
            self.code.get_lcp_mut(),
            &mut self.lcp_slots,
            &mut self.next_lcp_slot,
            &module_path,
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
    lcp: &'b mut LCP,
    lcp_slots: &'b mut HashMap<LCPKey, u32>,
    next_lcp_slot: &'b mut u32,
    locals: HashMap<String, LocalSlot>,
    next_local_index: u32,
    code: FunctionCode,
    module_path: &'b str,
}

impl<'b> FunctionCompiler<'b> {
    fn new(
        dependencies: &'b HashMap<Descriptor, ModuleSigniture>,
        body: &'b [Statement],
        signiture: &'b FunctionSigniture,
        lcp: &'b mut LCP,
        lcp_slots: &'b mut HashMap<LCPKey, u32>,
        next_lcp_slot: &'b mut u32,
        module_path: &'b str,
    ) -> Self {
        Self {
            dependencies,
            body,
            signiture,
            locals: HashMap::new(),
            code: FunctionCode::empty(),
            lcp,
            lcp_slots,
            next_lcp_slot,
            next_local_index: 0,
            module_path,
        }
    }

    fn emit(&mut self, instr: Instruction) {
        self.code.emit_instr(instr);
    }

    fn get_lcp_slot(&mut self, entry: LCPEntry) -> u32 {
        let key = lcp_entry_to_key(&entry);
        *self.lcp_slots.entry(key).or_insert_with(|| {
            let i = *self.next_lcp_slot;
            *self.next_lcp_slot += 1;
            self.lcp.add(entry);
            i
        })
    }

    fn get_local_slot(&mut self, name: &str) -> LocalSlot {
        *self.locals.entry(name.to_string()).or_insert_with(|| {
            let i = self.next_local_index;
            self.next_local_index += 1;
            LocalSlot { index: i as u32 }
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

            // Needs to create String and Array class first
            // And native functions ...
            ExprKind::StringLiteral(_) => todo!(),
            ExprKind::ArrayLiteral(_) => todo!(),

            ExprKind::Path(path) => {
                if path.len() > 1 {
                    return Err(CompileError::other_at(
                        "Path with multiple segments in expression is not allowed",
                        expr.span,
                    ));
                }

                let slot = self.get_local_slot(path.last().unwrap());
                self.emit(Instruction::LoadLocal { slot: slot.index });
            }

            ExprKind::BinaryOp(op, lhs, rhs) => {
                self.compile_expr(lhs)?;
                self.compile_expr(rhs)?;
                let op_instruction = bin_op_to_instruction(op, expr.ty.as_ref().unwrap());
                self.emit(op_instruction);
            }

            ExprKind::UnaryOp(op, expr) => {
                self.compile_expr(expr)?;
                let op_instruction = un_op_to_instruction(op, expr.ty.as_ref().unwrap());
                self.emit(op_instruction);
            }

            // TODO: optimize this
            ExprKind::FunctionCall { function, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                let path = if function.len() == 1 {
                    format!("{}.{}", self.module_path, function.to_string())
                } else {
                    function.to_string()
                };

                let function_entry = LCPEntry::UnresolvedFunction { path };
                let function_slot = self.get_lcp_slot(function_entry);

                self.emit(Instruction::FunctionCall(function_slot));
            }
        }

        Ok(())
    }
}

// TODO: refactor this shit
fn bin_op_to_instruction(op: &BinaryOp, result_type: &Descriptor) -> Instruction {
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

fn un_op_to_instruction(op: &UnaryOp, result_type: &Descriptor) -> Instruction {
    match op {
        UnaryOp::Negate if result_type.is_i32_type() => Instruction::I32Negate,
        UnaryOp::Negate if result_type.is_i32_type() => Instruction::F32Negate,
        UnaryOp::Not if result_type.is_bool_type() => Instruction::BoolNot,
        _ => panic!("Unsupported unary operation"),
    }
}

fn lcp_entry_to_key(entry: &LCPEntry) -> LCPKey {
    match entry.clone() {
        LCPEntry::ResolvedClass(_) | LCPEntry::ResolvedFunction(_) => unreachable!(),
        LCPEntry::String(str) => LCPKey::String(str),
        LCPEntry::UnresolvedClass { path } => LCPKey::Class(path),
        LCPEntry::UnresolvedFunction { path } => LCPKey::Function(path),
        LCPEntry::UnresolvedInterface { path } => LCPKey::Interface(path),
    }
}
