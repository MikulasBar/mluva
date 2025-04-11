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

                Instruction::Add => self.apply_bin_op(Value::add)?,
                Instruction::Sub => self.apply_bin_op(Value::sub)?,
                Instruction::Mul => self.apply_bin_op(Value::mul)?,
                Instruction::Div => self.apply_bin_op(Value::div)?,
                Instruction::Modulo => self.apply_bin_op(Value::modulo)?,
                Instruction::Equal => self.apply_bin_op(Value::equal)?,
                Instruction::NotEqual => self.apply_bin_op(Value::not_equal)?,
                Instruction::Less => self.apply_bin_op(Value::less)?,
                Instruction::LessEqual => self.apply_bin_op(Value::less_equal)?,
                Instruction::Greater => self.apply_bin_op(Value::greater)?,
                Instruction::GreaterEqual => self.apply_bin_op(Value::greater_equal)?,
                Instruction::And => self.apply_bin_op(Value::and)?,
                Instruction::Or => self.apply_bin_op(Value::or)?,
            }
            
            self.index += 1;
        }

        Ok(())
    }
    
    fn apply_bin_op(&mut self, op: fn(&Value, Value) -> Result<Value, InterpreterError>) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self.stack.last_mut().ok_or(InterpreterError::ValueStackUnderflow)?;

        *b = op(&*b, a)?;

        Ok(())
    }
}