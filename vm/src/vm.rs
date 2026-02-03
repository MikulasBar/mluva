use common::module::Module;
use common::vm::{Arena, CallFrame, ListObject, RuntimeError, StringObject, ValueStack};

use common::{
    function::FunctionCode,
    instruction::Instruction,
    module::module_manager::ModuleManager,
    type_manager::{LIST_TYPE_ID, STRING_TYPE_ID},
    word::Word,
};

pub struct Vm {
    pub value_stack: ValueStack,
    pub arena: Arena,
    pub callstack: Vec<CallFrame>,
    pub modules: ModuleManager,
}

impl Vm {
    pub fn new(modules: ModuleManager) -> Self {
        Self {
            value_stack: ValueStack::new(),
            arena: Arena::new(),
            callstack: vec![],
            modules: modules,
        }
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        let main_module_slot = self
            .modules
            .get_main_slot()
            .ok_or(RuntimeError::other("main module not found"))?;

        match self
            .modules
            .get_by_slot(main_module_slot)
            .ok_or(RuntimeError::other("main module not found"))?
        {
            Module::Compiled(m) => {
                let main_func = m
                    .get_main_code()
                    .ok_or(RuntimeError::other("main function not found"))?;
                FunctionInterpreter::new(
                    &self.modules,
                    &mut self.arena,
                    &mut self.value_stack,
                    main_func,
                    &mut self.callstack,
                    main_module_slot,
                )
                .execute()
            }

            Module::Native(_) => Err(RuntimeError::other("main module cannot be native")),
        }
    }
}

struct FunctionInterpreter<'a> {
    modules: &'a ModuleManager,
    arena: &'a mut Arena,
    value_stack: &'a mut ValueStack,
    code: &'a FunctionCode,
    callstack: &'a mut Vec<CallFrame>,
    current_module_slot: u32,
    ip: usize,
}

impl<'a> FunctionInterpreter<'a> {
    pub fn new(
        modules: &'a ModuleManager,
        arena: &'a mut Arena,
        value_stack: &'a mut ValueStack,
        code: &'a FunctionCode,
        callstack: &'a mut Vec<CallFrame>,
        current_module_slot: u32,
    ) -> Self {
        let callframe = CallFrame::new(code.slot_count as u32);
        callstack.push(callframe);

        Self {
            modules,
            arena,
            value_stack,
            code,
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
        while self.ip < self.code.len() {
            let instr = &self.code.get_instr(self.ip);
            match instr {
                Instruction::Store { slot } => {
                    let w = self.pop()?;
                    self.local_set(*slot, w)?;
                }
                Instruction::LoadConst(word) => {
                    self.value_stack.push(*word);
                }
                Instruction::Drop => {
                    self.pop()?;
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
                    let str = self
                        .modules
                        .get_string_from_pool(self.current_module_slot, *pool_slot)
                        .ok_or(RuntimeError::Unknown)?;

                    let string_object = StringObject::new(str);
                    let handle = self.arena.alloc(STRING_TYPE_ID, string_object);

                    self.push(handle.as_word());
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
                    self.arena.decrement_rc(
                        &handle,
                        self.callstack.last_mut().unwrap(),
                        self.value_stack,
                        self.modules,
                    )?;
                }
                Instruction::LocalCall { slot } => {
                    let func = self
                        .modules
                        .get_by_slot(self.current_module_slot)
                        .ok_or(RuntimeError::other("Module not found"))?
                        .get_code_by_slot(*slot)
                        .ok_or(RuntimeError::Unknown)?;

                    FunctionInterpreter::new(
                        self.modules,
                        self.arena,
                        self.value_stack,
                        func,
                        self.callstack,
                        self.current_module_slot,
                    )
                    .execute()?;
                }
                Instruction::ForeignCall {
                    module_name_slot,
                    call_slot,
                } => {
                    let mod_name = self
                        .modules
                        .get_string_from_pool(self.current_module_slot, *module_name_slot)
                        .ok_or(RuntimeError::Unknown)?;

                    let module_slot = self
                        .modules
                        .get_slot(mod_name)
                        .ok_or(RuntimeError::other("Module doesn't exists"))?;

                    let func = self
                        .modules
                        .get_by_slot(module_slot)
                        .ok_or(RuntimeError::other("Module not found"))?
                        .get_code_by_slot(*call_slot)
                        .ok_or(RuntimeError::Unknown)?;

                    FunctionInterpreter::new(
                        self.modules,
                        self.arena,
                        self.value_stack,
                        func,
                        self.callstack,
                        module_slot,
                    )
                    .execute()?;
                }
                // Instruction::MethodCall { type_id, slot } => {
                //     let callee = self.pop()?;
                //     let method = self.vtables[*type_id as usize]
                //         .methods
                //         .get(*slot as usize)
                //         .ok_or(RuntimeError::Unknown)?;

                //     method.execute(callee, self.value_stack, self.arena, self.vtables);
                // }
                Instruction::ListGet => {
                    let index = self.pop()?.as_u32();
                    let handle = self.pop()?.as_hhandle();
                    let list = self.arena.get_mut::<ListObject>(&handle)?;
                    let item = list.get_item(index)?;
                    self.arena.decrement_rc(
                        &handle,
                        self.callstack.last_mut().unwrap(),
                        self.value_stack,
                        self.modules,
                    )?;
                    self.push(item);
                }
                Instruction::ListSet => {
                    let value = self.pop()?;
                    let index = self.pop()?.as_u32();
                    let handle = self.pop()?.as_hhandle();
                    let list = self.arena.get_mut::<ListObject>(&handle)?;
                    list.set_item(index, value)?;
                    self.arena.decrement_rc(
                        &handle,
                        self.callstack.last_mut().unwrap(),
                        self.value_stack,
                        self.modules,
                    )?;
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
                Instruction::I32Mod => {
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
                Instruction::F32Mod => {
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
                Instruction::WordEqual => {
                    let rhs = self.pop()?;
                    self.last_mut()?.cmp_assign_equal(rhs);
                }
                Instruction::WordNotEqual => {
                    let rhs = self.pop()?;
                    self.last_mut()?.cmp_assign_not_equal(rhs);
                }
            }
            self.ip += 1;
        }

        Err(RuntimeError::FunctionDidNotReturn)
    }
}
