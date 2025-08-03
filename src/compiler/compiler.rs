use core::panic;
use std::collections::HashMap;

use crate::errors::CompileError;
use crate::executable_module::ExecutableModule;
use crate::function::{InternalFunctionSource};
use crate::ast::{BinaryOp, Expr, Item, Stmt, UnaryOp};
use crate::instruction::Instruction;

use crate::function::InternalFunctionDefinition;
use crate::value::Value;

use super::DataType;

pub struct Compiler {
    fn_map: HashMap<String, usize>,
    functions: Vec<Option<InternalFunctionSource>>,
    main_slot: Option<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            fn_map: HashMap::new(),
            main_slot: None,
        }
    }

    pub fn compile(mut self, items: &[Item]) -> Result<ExecutableModule, CompileError> {
        self.allocate_function_slots(items)?;
        self.compile_items(items)?;

        let mut functions = Vec::with_capacity(self.functions.len());


        // check if all functions are defined
        for (slot, f_opt) in self.functions.into_iter().enumerate() {
            if let Some(f) = f_opt {
                functions.push(f);
            } else {
                return Err(CompileError::FunctionNotFound(format!("slot({})", slot)));
            }
        }

        if self.main_slot.is_none() {
            return Err(CompileError::FunctionNotFound("main".to_string()));
        }

        Ok(ExecutableModule::new(functions, self.main_slot.unwrap()))
    }

    fn allocate_function_slots(&mut self, items: &[Item]) -> Result<(), CompileError> {
        let mut count = 0;

        for item in items {
            match item {
                Item::FunctionDef(InternalFunctionDefinition { name, .. }) => {
                    if self.fn_map.contains_key(name) {
                        return Err(CompileError::FunctionAlreadyDefined(name.clone()));
                    }

                    if name == "main" {
                        self.main_slot = Some(count);                        
                    }

                    self.fn_map.insert(name.clone(), count);
                    count += 1;
                },
            }
        }

        self.functions.resize(count, None);

        Ok(())
    }

    fn compile_items(&mut self, items: &[Item]) -> Result<(), CompileError> {
        for item in items {
            match item {
                Item::FunctionDef(fn_def) => {
                    let source = FunctionCompiler::new(self).compile(&fn_def)?;
                    let Some(slot) = self.fn_map.get(&fn_def.name) else {
                        // TODO: separate error for function not found and function not defined
                        return Err(CompileError::FunctionNotFound(fn_def.name.clone()));
                    };

                    if self.functions[*slot].is_some() {
                        return Err(CompileError::FunctionAlreadyDefined(fn_def.name.clone()));
                    }

                    self.functions[*slot] = Some(source);
                },
            }
        }

        Ok(())
    }
}

struct FunctionCompiler<'b> {
    compiler: &'b mut Compiler,
    instructions: Vec<Instruction>,
    locals: HashMap<String, usize>,
    next_slot: usize,
}

impl<'b> FunctionCompiler<'b> {
    fn new(compiler: &'b mut Compiler) -> Self {
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

    fn update_instruction_at(&mut self, index: usize, inst: Instruction) {
        let len = self.instructions.len();
        if index >= len {
            panic!("Index out of bounds :{}, length: {}", index, len);
        }

        self.instructions[index] = inst;
    }

    fn push(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    fn compile(mut self, def: &InternalFunctionDefinition) -> Result<InternalFunctionSource, CompileError> {
        for (name, _) in &def.params {
            let slot = self.get_slot(&name);
            self.instructions.push(Instruction::Store(slot as u32));
        }

        self.compile_stmts(&def.body)?;
        
        // implicit return at the end of Void functions
        if let DataType::Void = def.return_type {
            self.instructions.push(Instruction::Push(Value::Void));
            self.instructions.push(Instruction::Return);
        }

        Ok(InternalFunctionSource::new(self.locals.len(), self.instructions))
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
                let slot = self.get_slot(name);
                self.push(Instruction::Store(slot as u32));
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
                let slot = self.get_slot(name);
                self.instructions.push(Instruction::Load(slot as u32));
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

            Expr::FuncCall(name, args) => {
                for arg in args {
                    self.compile_expr(arg)?;
                }

                let Some(&slot) = self.compiler.fn_map.get(name) else {
                    panic!("Function {} not found", name);
                };

                self.instructions.push(Instruction::Call {
                    slot: slot as u32,
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
