use std::collections::HashMap;

use crate::{
    arena::Arena,
    builtin_function::BuiltinFunction,
    callframe::CallFrame,
    errors::RuntimeError,
    function::FunctionSource,
    instruction::Instruction,
    list_object::ListObject,
    module::Module,
    value_stack::ValueStack,
    vtable::{VTable, LIST_TYPE_ID},
    word::Word,
};

pub struct Vm {
    pub value_stack: ValueStack,
    pub arena: Arena,
    pub vtables: Vec<VTable>,
    pub call_stack: Vec<CallFrame>,
    pub modules: HashMap<String, Module>,
    pub main_module: Module,
}

impl Vm {
    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        let main_source = self
            .main_module
            .get_main_source()
            .ok_or(RuntimeError::Other("Main function not found".to_string()))?;

        self.call_stack
            .push(CallFrame::new(main_source.slot_count as u32));

        let callframe = self.call_stack.last_mut().ok_or(RuntimeError::Unknown)?;

        FunctionInterpreter::new(
            &mut self.modules,
            &mut self.arena,
            &mut self.value_stack,
            &mut self.vtables,
            main_source,
            callframe,
        )
        .execute()
    }
}

struct FunctionInterpreter<'a> {
    modules: &'a HashMap<String, Module>,
    arena: &'a mut Arena,
    value_stack: &'a mut ValueStack,
    vtables: &'a Vec<VTable>,
    source: &'a FunctionSource,
    callframe: &'a mut CallFrame,
    ip: usize,
}

impl<'a> FunctionInterpreter<'a> {
    pub fn new(
        modules: &'a HashMap<String, Module>,
        arena: &'a mut Arena,
        value_stack: &'a mut ValueStack,
        vtables: &'a Vec<VTable>,
        source: &'a FunctionSource,
        callframe: &'a mut CallFrame,
    ) -> Self {
        Self {
            modules,
            arena,
            value_stack,
            vtables,
            source,
            callframe,
            ip: 0,
        }
    }

    fn push(&mut self, value: Word) {
        self.value_stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Word, RuntimeError> {
        self.value_stack.pop()
    }

    fn last_mut(&mut self) -> Result<&mut Word, RuntimeError> {
        self.value_stack.last_mut()
    }

    fn local_get(&mut self, slot: u32) -> Result<Word, RuntimeError> {
        self.callframe
            .locals
            .get(slot as usize)
            .copied()
            .ok_or(RuntimeError::Unknown)
    }

    fn local_set(&mut self, slot: u32, value: Word) -> Result<(), RuntimeError> {
        if let Some(local) = self.callframe.locals.get_mut(slot as usize) {
            *local = value;
            Ok(())
        } else {
            Err(RuntimeError::Unknown)
        }
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        while self.ip < self.source.body.len() {
            let instr = &self.source.body[self.ip];
            match instr {
                Instruction::Store { slot } => {
                    let w = self.pop()?;
                    self.local_set(*slot, w)?;
                }
                Instruction::LoadConst(word) => {
                    self.value_stack.push(*word);
                }
                Instruction::Pop => {
                    self.pop();
                }
                Instruction::LoadLocal { slot } => {
                    let value = self.local_get(*slot)?;
                    self.value_stack.push(value);
                }
                Instruction::Return => {
                    return Ok(());
                }
                Instruction::Jump(target) => {
                    self.ip = *target as usize;
                    continue;
                }
                Instruction::JumpIfFalse(target) => {
                    let condition = self.pop()?;
                    if !condition.as_bool() {
                        self.ip = *target as usize;
                        continue;
                    }
                }
                Instruction::BoolAnd => {
                    let rhs = self.pop()?;
                    self.last_mut()?.bool_assign_and(rhs);
                }
                Instruction::BoolOr => {
                    let rhs = self.pop()?;
                    self.last_mut()?.bool_assign_or(rhs);
                }
                Instruction::BoolNot => {
                    self.last_mut()?.bool_assign_not();
                }
                Instruction::I32Add => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_add(rhs);
                }
                Instruction::I32Sub => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_sub(rhs);
                }
                Instruction::I32Mul => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_mul(rhs);
                }
                Instruction::I32Div => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_div(rhs);
                }
                Instruction::I32Modulo => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_modulo(rhs);
                }
                Instruction::I32Negate => {
                    self.last_mut()?.i32_assign_negate();
                }
                Instruction::I32Greater => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_greater(rhs);
                }
                Instruction::I32GreaterEqual => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_greater_equal(rhs);
                }
                Instruction::I32Less => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_less(rhs);
                }
                Instruction::I32LessEqual => {
                    let rhs = self.pop()?;
                    self.last_mut()?.i32_assign_less_equal(rhs);
                }
                Instruction::F32Add => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_add(rhs);
                }
                Instruction::F32Sub => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_sub(rhs);
                }
                Instruction::F32Mul => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_mul(rhs);
                }
                Instruction::F32Div => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_div(rhs);
                }
                Instruction::F32Modulo => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_modulo(rhs);
                }
                Instruction::F32Negate => {
                    self.last_mut()?.f32_assign_negate();
                }
                Instruction::F32Greater => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_greater(rhs);
                }
                Instruction::F32GreaterEqual => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_greater_equal(rhs);
                }
                Instruction::F32Less => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_less(rhs);
                }
                Instruction::F32LessEqual => {
                    let rhs = self.pop()?;
                    self.last_mut()?.f32_assign_less_equal(rhs);
                }
                Instruction::Equal => {
                    let rhs = self.pop()?;
                    self.last_mut()?.cmp_assign_equal(rhs);
                }
                Instruction::NotEqual => {
                    let rhs = self.pop()?;
                    self.last_mut()?.cmp_assign_not_equal(rhs);
                }
                Instruction::CreateString { pool_slot } => {
                    todo!("CreateString not implemented yet");
                }
                Instruction::CreateList {
                    item_count,
                    type_id,
                } => {
                    let count = *item_count as usize;
                    let items = self.value_stack.split_off(self.value_stack.len() - count);
                    let list_object = ListObject::from_values(*type_id, items);
                    let handle = self.arena.alloc(LIST_TYPE_ID, list_object);
                    self.push(handle.to_word());
                }
                Instruction::RcInc => {
                    self.pop()?.hhandle_rc_inc(self.arena)?;
                }
                Instruction::RcDec => {
                    self.pop()?.hhandle_rc_dec(self.arena, self.vtables)?;
                }
                Instruction::BuiltinFunctionCall { slot, argc } => {
                    BuiltinFunction::execute(*slot, *argc, self.value_stack, self.arena)?;
                }
                Instruction::LocalCall { slot } => {
                    let func = current_module
                        .get_function_source_by_slot(*slot)
                        .ok_or(RuntimeError::Unknown)?;

                    self.execute_function(func, current_module)?;
                }
                Instruction::ForeignCall {
                    ref module_name,
                    call_slot,
                } => {
                    let module = self.modules.get(module_name).ok_or(RuntimeError::Unknown)?;

                    let func = module
                        .get_function_source_by_slot(*call_slot)
                        .ok_or(RuntimeError::Unknown)?;

                    FunctionInterpreter::new(
                        self.modules,
                        self.arena,
                        self.value_stack,
                        self.vtables,
                        func,
                        &mut CallFrame::new(func.slot_count as u32),
                    )
                }
                Instruction::MethodCall {
                    type_id,
                    slot,
                    argc,
                } => {
                    let method = self.vtables[*type_id as usize]
                        .methods
                        .get(*slot as usize)
                        .ok_or(RuntimeError::Unknown)?;

                    method.execute(self.vtables, self.arena);
                }
                Instruction::ListGet => {
                    let index = self.pop()?.as_u32();
                    let list_handle = self.pop()?.as_hhandle();
                    let list_object = self.arena.get_mut::<ListObject>(&list_handle)?;
                    let item = list_object.get_item(index)?;
                    self.push(item);
                }
                Instruction::ListSet => {
                    let value = self.pop()?;
                    let index = self.pop()?.as_u32();
                    let list_handle = self.pop()?.as_hhandle();
                    let list_object = self.arena.get_mut::<ListObject>(&list_handle)?;

                    list_object.set_item(index, value)?;
                }
            }
            self.ip += 1;
        }

        Err(RuntimeError::FunctionDidNotReturn)
    }
}
