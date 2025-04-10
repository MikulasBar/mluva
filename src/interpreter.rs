use crate::errors::InterpreterError;
use crate::function_table::FunctionTable;
use crate::instruction::Instruction;
use crate::interpreter_source::InterpreterSource;
use crate::value::Value;

pub struct Interpreter<'a> {
    function_table: &'a FunctionTable,
    index: usize,
    stack: Vec<Value>,
    instructions: Vec<Instruction>,
    slots: Vec<Value>,
}

impl<'a> Interpreter<'a> {
    pub fn new(function_table: &'a FunctionTable, source: InterpreterSource) -> Self {
        Self {
            function_table,
            instructions: source.instructions,
            index: 0,
            stack: vec![],
            slots: vec![Value::Void; source.local_slots],
        }
    }

    fn pop(&mut self) -> Result<Value, InterpreterError> {
        self.stack.pop().ok_or(InterpreterError::ValueStackUnderflow)
    }

    pub fn interpret(&mut self) -> Result<(), InterpreterError> {
        while self.index < self.instructions.len() {
            let instruction = &self.instructions[self.index];
            match *instruction {
                Instruction::Push(ref value) => {
                    self.stack.push(value.clone());
                }

                Instruction::Pop => {
                    self.pop()?;
                }

                Instruction::Store(slot) => {
                    self.slots[slot] = self.pop()?;
                }

                Instruction::Load(slot) => {
                    self.stack.push(self.slots[slot].clone());
                }

                Instruction::Call { slot, arg_count } => {
                    let args = self.stack.split_off(self.stack.len() - arg_count);
                    let func = self.function_table.get_fn_by_index(slot).unwrap();
                    let result = func.call(args)?;
                    self.stack.push(result);
                }
                
                Instruction::Return => {
                    break; // TODO: Handle return logic here
                }

                Instruction::Jump(target) => {
                    // Jump to target
                    self.index = target;
                    continue; // Skip the index increment below
                }

                Instruction::JumpIfFalse(target) => {
                    let cond = self.pop()?;

                    if cond.is_false()? {
                        self.index = target;
                        continue; // Skip the index increment below
                    }
                }

                Instruction::Add => self.apply_assign_op(Value::add_assign)?,
                Instruction::Sub => self.apply_assign_op(Value::sub_assign)?,
                Instruction::Mul => self.apply_assign_op(Value::mul_assign)?,
                Instruction::Div => self.apply_assign_op(Value::div_assign)?,
                Instruction::Modulo => self.apply_assign_op(Value::modulo_assign)?,
                Instruction::Equal => self.apply_assign_op(|a, b| {
                    *a = Value::Bool(*a == b);
                    Ok(())
                })?,

                Instruction::NotEqual => self.apply_assign_op(|a, b| {
                    *a = Value::Bool(*a != b);
                    Ok(())
                })?,

                            
            }
            self.index += 1;
        }

        Ok(())
    }

    
    fn apply_assign_op(&mut self, op: fn(&mut Value, Value) -> Result<(), InterpreterError>) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.stack.last_mut().ok_or(InterpreterError::ValueStackUnderflow)?;

        op(b, a)
    }
}