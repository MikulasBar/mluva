use crate::{
    arena::Arena,
    builtin_function::BuiltinFunction,
    callframe::CallFrame,
    errors::RuntimeError,
    function::FunctionSource,
    instruction::Instruction,
    list_object::ListObject,
    module_cluster::ModuleCluster,
    value_stack::ValueStack,
    vtable::{VTable, LIST_TYPE_ID, STRING_TYPE_ID},
    word::Word,
};

pub struct Vm {
    pub value_stack: ValueStack,
    pub arena: Arena,
    pub vtables: Vec<VTable>,
    pub callstack: Vec<CallFrame>,
    pub modules: ModuleCluster,
}

impl Vm {
    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        let main_module_slot = self
            .modules
            .get_main_module_slot()
            .ok_or(RuntimeError::other("main module not found"))?;

        let main_source = self
            .modules
            .get_by_slot(main_module_slot)
            .ok_or(RuntimeError::other("main module not found"))?
            .get_main_source()
            .ok_or(RuntimeError::other("Main function not found"))?;

        FunctionInterpreter::new(
            &self.modules,
            &mut self.arena,
            &mut self.value_stack,
            &mut self.vtables,
            main_source,
            &mut self.callstack,
            main_module_slot,
        )
        .execute()
    }
}

struct FunctionInterpreter<'a> {
    modules: &'a ModuleCluster,
    arena: &'a mut Arena,
    value_stack: &'a mut ValueStack,
    vtables: &'a Vec<VTable>,
    source: &'a FunctionSource,
    callstack: &'a mut Vec<CallFrame>,
    current_module_slot: usize,
    ip: usize,
}

impl<'a> FunctionInterpreter<'a> {
    pub fn new(
        modules: &'a ModuleCluster,
        arena: &'a mut Arena,
        value_stack: &'a mut ValueStack,
        vtables: &'a Vec<VTable>,
        source: &'a FunctionSource,
        callstack: &'a mut Vec<CallFrame>,
        current_module_slot: usize,
    ) -> Self {
        let callframe = CallFrame::new(source.slot_count as u32);
        callstack.push(callframe);

        Self {
            modules,
            arena,
            value_stack,
            vtables,
            source,
            callstack,
            current_module_slot,
            ip: 0,
        }
    }

    fn locals_mut(&mut self) -> &mut Vec<Word> {
        &mut self.callstack.last_mut().unwrap().locals
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

    fn copy_last(&mut self) -> Result<Word, RuntimeError> {
        self.value_stack.copy_last()
    }

    fn local_get(&mut self, slot: u32) -> Result<Word, RuntimeError> {
        self.locals_mut()
            .get(slot as usize)
            .copied()
            .ok_or(RuntimeError::Unknown)
    }

    fn local_set(&mut self, slot: u32, value: Word) -> Result<(), RuntimeError> {
        if let Some(local) = self.locals_mut().get_mut(slot as usize) {
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
                    self.callstack.pop();
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
                    self.push(handle.as_word());
                }
                Instruction::RcInc => {
                    let handle = self.copy_last()?.as_hhandle();
                    self.arena.increment_rc(&handle)?;
                }
                Instruction::RcDec => {
                    let handle = self.pop()?.as_hhandle();
                    self.arena
                        .decrement_rc(&handle, self.value_stack, &self.vtables)?;
                }
                Instruction::BuiltinFunctionCall { slot, argc } => {
                    BuiltinFunction::execute(*slot, *argc, self.value_stack, self.arena)?;
                }
                Instruction::LocalCall { slot } => {
                    let func = self
                        .modules
                        .get_by_slot(self.current_module_slot)
                        .ok_or(RuntimeError::other("Module not found"))?
                        .get_function_source_by_slot(*slot)
                        .ok_or(RuntimeError::Unknown)?;

                    FunctionInterpreter::new(
                        self.modules,
                        self.arena,
                        self.value_stack,
                        self.vtables,
                        func,
                        self.callstack,
                        self.current_module_slot,
                    )
                    .execute()?;
                }
                Instruction::ForeignCall {
                    ref module_name,
                    call_slot,
                } => {
                    let module_slot = self
                        .modules
                        .get_slot(module_name)
                        .ok_or(RuntimeError::other("Module doesn't exists"))?;

                    let func = self
                        .modules
                        .get_by_slot(module_slot)
                        .ok_or(RuntimeError::other("Module not found"))?
                        .get_function_source_by_slot(*call_slot)
                        .ok_or(RuntimeError::Unknown)?;

                    FunctionInterpreter::new(
                        self.modules,
                        self.arena,
                        self.value_stack,
                        self.vtables,
                        func,
                        self.callstack,
                        module_slot,
                    )
                    .execute()?;
                }
                Instruction::MethodCall { type_id, slot } => {
                    let callee = self.pop()?;
                    let method = self.vtables[*type_id as usize]
                        .methods
                        .get(*slot as usize)
                        .ok_or(RuntimeError::Unknown)?;

                    method.execute(callee, self.value_stack, self.arena, self.vtables);
                }
                Instruction::ListGet => {
                    let index = self.pop()?.as_u32();
                    let list_handle = self.pop()?.as_hhandle();
                    let list = self.arena.get_mut::<ListObject>(&list_handle)?;
                    let item = list.get_item(index)?;
                    self.push(item);
                }
                Instruction::ListSet => {
                    let value = self.pop()?;
                    let index = self.pop()?.as_u32();
                    let list_handle = self.pop()?.as_hhandle();
                    let list = self.arena.get_mut::<ListObject>(&list_handle)?;

                    list.set_item(index, value)?;
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
            }
            self.ip += 1;
        }

        Err(RuntimeError::FunctionDidNotReturn)
    }
}
