use crate::errors::RuntimeError;
use crate::function::{InternalFunctionSource};
use crate::instruction::Instruction;
use crate::module::Module;
use crate::value::Value;

pub struct Interpreter<'a> {
    module: &'a Module,
    stack: Vec<Value>,
}

impl<'a> Interpreter<'a> {
    pub fn new(module: &'a Module) -> Self {
        Self {
            module,
            stack: vec![],
        }
    }

    pub fn execute(mut self) -> Result<Value, RuntimeError> {
        let main_function = self.module.get_main_source().ok_or(
            RuntimeError::Other("Module is not executable (missing main function)".to_string()),
        )?;

        let functions = self.module.get_sources();
        let val = InternalFunctionInterpreter::new(functions, &mut self.stack, main_function).interpret()?;
        
        Ok(val)
    }

}

struct InternalFunctionInterpreter<'a> {
    functions: &'a [InternalFunctionSource],
    stack: &'a mut Vec<Value>,
    index: usize,
    slots: Vec<Value>,
    source: &'a InternalFunctionSource,
}

impl<'a> InternalFunctionInterpreter<'a> {
    pub fn new(
        functions: &'a [InternalFunctionSource],
        stack: &'a mut Vec<Value>,
        source: &'a InternalFunctionSource,
    ) -> Self {
        Self {
            functions,
            stack,
            source,
            index: 0,
            slots: vec![Value::Void; source.slot_count],
        }
    }

    fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.stack
            .pop()
            .ok_or(RuntimeError::ValueStackUnderflow)
    }

    pub fn interpret(mut self) -> Result<Value, RuntimeError> {
        while self.index < self.source.body.len() {
            let instruction = &self.source.body[self.index];
            match *instruction {
                Instruction::Push(ref value) => {
                    self.stack.push(value.clone());
                }

                Instruction::Pop => {
                    self.pop()?;
                }

                Instruction::Store { slot } => {
                    self.slots[slot as usize] = self.pop()?;
                }

                Instruction::Load { slot } => {
                    self.stack.push(self.slots[slot as usize].clone());
                }

                Instruction::Call { call_slot } => {
                    let source = &self.functions[call_slot as usize];
                    let result =
                        InternalFunctionInterpreter::new(self.functions, &mut self.stack, source).interpret()?;
                    self.stack.push(result);
                }

                Instruction::Return => {
                    return Ok(self.pop()?);
                }

                Instruction::Jump(target) => {
                    self.index = target as usize;
                    continue; // Skip the index increment below
                }

                Instruction::JumpIfFalse(target) => {
                    let cond = self.pop()?;

                    if cond.is_false()? {
                        self.index = target as usize;
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

                Instruction::Not => self.apply_un_op(Value::not)?,
                Instruction::Negate => self.apply_un_op(Value::negate)?,
            }

            self.index += 1;
        }

        Err(RuntimeError::FunctionDidNotReturn)
    }

    fn apply_bin_op(
        &mut self,
        op: fn(&Value, Value) -> Result<Value, RuntimeError>,
    ) -> Result<(), RuntimeError> {
        let a = self.pop()?;
        let b = self
            .stack
            .last_mut()
            .ok_or(RuntimeError::ValueStackUnderflow)?;

        *b = op(&*b, a)?;

        Ok(())
    }

    fn apply_un_op(
        &mut self,
        op: fn(&Value) -> Result<Value, RuntimeError>,
    ) -> Result<(), RuntimeError> {
        let a = self
            .stack
            .last_mut()
            .ok_or(RuntimeError::ValueStackUnderflow)?;

        *a = op(&*a)?;

        Ok(())
    }
}

fn get_args_from_stack(
    stack: &mut Vec<Value>,
    arg_count: usize,
) -> Result<Vec<Value>, RuntimeError> {
    if stack.len() < arg_count {
        return Err(RuntimeError::ValueStackUnderflow);
    }

    // Split the stack to get the arguments
    let args = stack.split_off(stack.len() - arg_count);
    // Reverse the arguments to maintain the order
    let args = args.into_iter().rev().collect::<Vec<_>>();
    Ok(args)
}
