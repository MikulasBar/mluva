use core::panic;
use std::collections::HashMap;

use common::ast::{
    BinaryOp, Expr, ExprKind, FunctionSigniture, Parameter, Pattern, PatternKind, Statement,
    StatementKind, UnaryOp,
};
use common::compile_error::CompileError;
use common::data_type::DataType;
use common::function_source::FunctionSource;
use common::instruction::Instruction;
use common::module::module_signiture::ModuleSigniture;
use common::type_manager::{BOOL_TYPE_ID, F32_TYPE_ID, I32_TYPE_ID};
use common::word::Word;

pub struct Compiler<'a> {
    sources: Vec<FunctionSource>,
    string_pool: HashMap<String, usize>,
    ast: Ast,
    dependencies: &'a HashMap<String, ModuleSigniture>,
    next_string_slot: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(ast: Ast, dependencies: &'a HashMap<String, ModuleSigniture>) -> Self {
        let function_count = ast.function_count() as usize;

        Self {
            sources: Vec::with_capacity(function_count),
            string_pool: HashMap::new(),
            ast,
            dependencies,
            next_string_slot: 0,
        }
    }

    pub fn compile(mut self) -> Result<ModuleSigniture, CompileError> {
        for slot in 0..self.ast.function_count() {
            self.compile_function(slot)?;
        }

        let (function_map, spanned_function_signitures, ..) = self.ast.deconstruct();
        // let function_signitures = spanned_function_signitures
        //     .into_iter()
        //     .map(Into::<>::into)
        // .collect();

        let main_slot = function_map.get("main").copied();
        todo!()
        // let module =
        //     ModuleSigniture::new(main_slot, function_map, function_signitures, self.sources);

        // Ok(module)
    }

    fn compile_function(&mut self, slot: u32) -> Result<(), CompileError> {
        let function_map = self.ast.get_function_map();
        let signiture = self.ast.get_function_signiture_by_slot(slot).unwrap();
        let body = self.ast.get_function_body_by_slot(slot).unwrap();

        todo!()
        // let source =
        //     FunctionCompiler::new(self.dependencies, function_map, body, signiture).compile()?;

        // self.sources.push(source);

        // Ok(())
    }
}

struct FunctionCompiler<'b> {
    dependencies: &'b HashMap<String, ModuleSigniture>,
    function_map: &'b HashMap<String, u32>,
    body: &'b [Statement],
    signiture: &'b SpannedFunctionSigniture,

    string_pool: &'b mut HashMap<String, usize>,
    instructions: Vec<Instruction>,
    locals: HashMap<String, usize>,
    next_local_slot: usize,
    next_string_slot: &'b mut usize,
}

impl<'b> FunctionCompiler<'b> {
    fn new(
        dependencies: &'b HashMap<String, ModuleSigniture>,
        function_map: &'b HashMap<String, u32>,
        body: &'b [Statement],
        signiture: &'b SpannedFunctionSigniture,
        string_pool: &'b mut HashMap<String, usize>,
        next_string_slot: &'b mut usize,
    ) -> Self {
        Self {
            dependencies,
            function_map,
            body,
            signiture,
            locals: HashMap::new(),
            instructions: Vec::new(),
            string_pool,
            next_local_slot: 0,
            next_string_slot,
        }
    }

    fn get_string_slot(&mut self, string: &str) -> usize {
        *self
            .string_pool
            .entry(string.to_string())
            .or_insert_with(|| {
                let slot = *self.next_string_slot;
                *self.next_string_slot += 1;
                slot
            })
    }

    fn get_local_slot(&mut self, name: &str) -> usize {
        *self.locals.entry(name.to_string()).or_insert_with(|| {
            let slot = self.next_local_slot;
            self.next_local_slot += 1;
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
            self.instructions.push(Instruction::LoadConst(Word::void()));
            self.instructions.push(Instruction::Return);
        }

        Ok(FunctionSource::new(self.locals.len(), self.instructions))
    }

    fn setup_parameters(&mut self) {
        for SpannedParameter { name, .. } in &self.signiture.params {
            let slot = self.get_local_slot(&name) as u32;
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
                assignee, value, ..
            }
            | StatementKind::VarAssign { assignee, value } => {
                self.compile_expr(&value)?;
                self.compile_pattern(&assignee)?;
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

    fn compile_pattern(&mut self, pattern: &Pattern) -> Result<(), CompileError> {
        match &pattern.kind {
            PatternKind::Variable(var) => {
                let slot = self.get_local_slot(&var) as u32;
                self.instructions.push(Instruction::Store { slot });
            }
            PatternKind::Index { callee, index } => {
                self.compile_expr(&*index)?;
                self.compile_pattern(&*callee);

                todo!()
            }
        }

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match &expr.kind {
            ExprKind::VoidLiteral => {
                self.instructions.push(Instruction::LoadConst(Word::void()));
            }

            ExprKind::IntLiteral(val) => {
                self.instructions
                    .push(Instruction::LoadConst(Word::from_i32(*val)));
            }

            ExprKind::FloatLiteral(val) => {
                self.instructions
                    .push(Instruction::LoadConst(Word::from_f32(*val)));
            }

            ExprKind::BoolLiteral(val) => {
                self.instructions
                    .push(Instruction::LoadConst(Word::from_bool(*val)));
            }

            ExprKind::StringLiteral(val) => {
                let slot = self.get_string_slot(val) as u32;
                self.instructions
                    .push(Instruction::CreateString { pool_slot: slot });
            }

            ExprKind::ListLiteral(list) => {
                for element in list {
                    self.compile_expr(element)?;
                }

                todo!()
            }

            ExprKind::Var(name) => {
                let slot = self.get_local_slot(name) as u32;
                self.instructions.push(Instruction::LoadLocal { slot });
            }

            ExprKind::BinaryOp(op, lhs, rhs) => {
                self.compile_expr(lhs)?;
                self.compile_expr(rhs)?;
                let op_instruction = bin_op_to_instruction(op, todo!());
                self.instructions.push(op_instruction);
            }

            ExprKind::UnaryOp(op, expr) => {
                self.compile_expr(expr)?;
                let op_instruction = un_op_to_instruction(op, todo!());
                self.instructions.push(op_instruction);
            }

            ExprKind::IndexGet { callee, index } => {
                self.compile_expr(callee)?;
                self.compile_expr(index)?;
                self.instructions.push(Instruction::ListGet);
            }

            ExprKind::FunctionCall { func_name, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                let Some(slot) = self.function_map.get(func_name).copied() else {
                    panic!(
                        "Function {} not found, should be handled in typechecker",
                        func_name
                    );
                };

                self.instructions.push(Instruction::LocalCall { slot });
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

            ExprKind::MethodCall {
                callee,
                method_name,
                args,
            } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                self.compile_expr(callee)?;

                todo!()
            }
        }

        Ok(())
    }
}

fn bin_op_to_instruction(op: &BinaryOp, result_type: u32) -> Instruction {
    match (op, result_type) {
        (BinaryOp::Add, I32_TYPE_ID) => Instruction::I32Add,
        (BinaryOp::Sub, I32_TYPE_ID) => Instruction::I32Sub,
        (BinaryOp::Mul, I32_TYPE_ID) => Instruction::I32Mul,
        (BinaryOp::Div, I32_TYPE_ID) => Instruction::I32Div,
        (BinaryOp::Modulo, I32_TYPE_ID) => Instruction::I32Mod,
        (BinaryOp::Less, I32_TYPE_ID) => Instruction::I32Less,
        (BinaryOp::LessEqual, I32_TYPE_ID) => Instruction::I32LessEqual,
        (BinaryOp::Greater, I32_TYPE_ID) => Instruction::I32Greater,
        (BinaryOp::GreaterEqual, I32_TYPE_ID) => Instruction::I32GreaterEqual,

        (BinaryOp::Add, F32_TYPE_ID) => Instruction::F32Add,
        (BinaryOp::Sub, F32_TYPE_ID) => Instruction::F32Sub,
        (BinaryOp::Mul, F32_TYPE_ID) => Instruction::F32Mul,
        (BinaryOp::Div, F32_TYPE_ID) => Instruction::F32Div,
        (BinaryOp::Modulo, F32_TYPE_ID) => Instruction::F32Mod,
        (BinaryOp::Less, F32_TYPE_ID) => Instruction::F32Less,
        (BinaryOp::LessEqual, F32_TYPE_ID) => Instruction::F32LessEqual,
        (BinaryOp::Greater, F32_TYPE_ID) => Instruction::F32Greater,
        (BinaryOp::GreaterEqual, F32_TYPE_ID) => Instruction::F32GreaterEqual,

        (BinaryOp::And, BOOL_TYPE_ID) => Instruction::BoolAnd,
        (BinaryOp::Or, BOOL_TYPE_ID) => Instruction::BoolOr,

        (BinaryOp::Equal, _) => Instruction::WordEqual,
        (BinaryOp::NotEqual, _) => Instruction::WordNotEqual,

        _ => panic!("Unsupported binary operation"),
    }
}

fn un_op_to_instruction(op: &UnaryOp, result_type: u32) -> Instruction {
    match (op, result_type) {
        (UnaryOp::Negate, I32_TYPE_ID) => Instruction::I32Negate,
        (UnaryOp::Negate, F32_TYPE_ID) => Instruction::F32Negate,
        (UnaryOp::Not, BOOL_TYPE_ID) => Instruction::BoolNot,
        _ => panic!("Unsupported unary operation"),
    }
}
