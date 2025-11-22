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
    vtable::{VTable, LIST_TYPE_ID},
    word::Word,
};

pub struct Vm {
    pub value_stack: Vec<Word>,
    pub arena: Arena,
    pub vtables: Vec<VTable>,
    pub call_stack: Vec<CallFrame>,
    pub modules: HashMap<String, Module>,
    pub main_module: Module,
}

impl Vm {
    pub fn execute(&mut self) {}

    fn push(&mut self, value: Word) {
        self.value_stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Word, RuntimeError> {
        self.value_stack
            .pop()
            .ok_or(RuntimeError::ValueStackUnderflow)
    }

    fn last_mut(&mut self) -> Result<&mut Word, RuntimeError> {
        self.value_stack
            .last_mut()
            .ok_or(RuntimeError::ValueStackUnderflow)
    }

    fn last_frame_mut(&mut self) -> Result<&mut CallFrame, RuntimeError> {
        self.call_stack.last_mut().ok_or(RuntimeError::Unknown)
    }

    fn local_get(&mut self, slot: u32) -> Result<Word, RuntimeError> {
        self.last_frame_mut()?
            .locals
            .get(slot as usize)
            .copied()
            .ok_or(RuntimeError::Unknown)
    }

    fn local_set(&mut self, slot: u32, value: Word) -> Result<(), RuntimeError> {
        if let Some(local) = self.last_frame_mut()?.locals.get_mut(slot as usize) {
            *local = value;
            Ok(())
        } else {
            Err(RuntimeError::Unknown)
        }
    }

    pub fn execute_function(
        &mut self,
        source: &FunctionSource,
        current_module: &Module,
    ) -> Result<(), RuntimeError> {
        let mut ip = 0;
        while ip < source.body.len() {
            let instr = &source.body[ip];
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
                    ip = *target as usize;
                    continue;
                }
                Instruction::JumpIfFalse(target) => {
                    let condition = self.pop()?;
                    if !condition.as_bool() {
                        ip = *target as usize;
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
                    self.pop()?.hhandle_rc_inc(&mut self.arena)?;
                }
                Instruction::RcDec => {
                    self.pop()?.hhandle_rc_dec(self)?;
                }
                Instruction::BuiltinFunctionCall { slot, argc } => {
                    BuiltinFunction::execute(*slot, *argc, self)?;
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

                    self.execute_function(func, module)?;
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

                    method.execute(self);
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
            ip += 1;
        }

        Err(RuntimeError::FunctionDidNotReturn)
    }
}
