use crate::errors::InterpreterError;
use crate::function::FunctionSource;
use crate::instruction::Instruction;
use crate::interpreter_source::InterpreterSource;
use crate::value::Value;

// const DEFAULT_STACK_SIZE: usize = 256;
// const DEFAULT_VALUE_STACK_SIZE: usize = 256;


// TODO: IMPLEMENT THE INTERPRETER
pub struct Interpreter {
    // indicates the current instruction
    index: usize,
    // stack for temporary values
    // used for intermediate calculations
    value_stack: Vec<Value>,

    slots: Vec<Value>,

    // // The call stack used for function calls and local variables
    // // all variables are stored there
    // // all functions have frames in the call stack
    // stack: Vec<Value>,
    // // indicates end of the stack
    // stack_index: usize,
    // // stores all indices of the frames in the call stack
    // // used to return to the correct frame after function returns 
    // frame_indices: Vec<usize>,
    // // source of the instructions
    // source: InterpreterSource,

    instructions: Vec<Instruction>,
    functions: Vec<FunctionSource>,
}

impl Interpreter {
    pub fn new(source: InterpreterSource) -> Self {
        let InterpreterSource { functions, main_slot } = source;
        // println!("FUNCTION SOURCES: {:?}\n", functions);
        // println!("MAIN: {:?}\n", main_slot);
        
        let main_source = functions[main_slot].clone();

        let FunctionSource::Internal(f) = main_source else {
            panic!("Main function must be internal")
        };

        Self {
            instructions: f.body,
            functions: functions,
            index: 0,
            value_stack: vec![],
            slots: vec![Value::Void; f.slot_count],
        }
    }

    fn pop(&mut self) -> Result<Value, InterpreterError> {
        self.value_stack
            .pop()
            .ok_or(InterpreterError::ValueStackUnderflow)
    }

    pub fn interpret(&mut self) -> Result<(), InterpreterError> {
        self.interpret_instructions()
    }

    pub fn interpret_instructions(&mut self) -> Result<(), InterpreterError> {
        while self.index < self.instructions.len() {
            let instruction = &self.instructions[self.index];
            match *instruction {
                Instruction::Push(ref value) => {
                    self.value_stack.push(value.clone());
                }

                Instruction::Pop => {
                    self.pop()?;
                }

                Instruction::Store(slot) => {
                    self.slots[slot] = self.pop()?;
                }

                Instruction::Load(slot) => {
                    self.value_stack.push(self.slots[slot].clone());
                }

                Instruction::Call { slot, arg_count } => {
                    let args = self.value_stack.split_off(self.value_stack.len() - arg_count);
                    let fn_source = &self.functions[slot];
                    let FunctionSource::External(func) = fn_source else {
                        panic!("Only external functions can be called for now")
                    };
                    let result = func.call(args)?;
                    self.value_stack.push(result);
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

                Instruction::Not => self.apply_un_op(Value::not)?,
                Instruction::Negate => self.apply_un_op(Value::negate)?,
            }

            self.index += 1;
        }

        Ok(())
    }

    fn apply_bin_op(
        &mut self,
        op: fn(&Value, Value) -> Result<Value, InterpreterError>,
    ) -> Result<(), InterpreterError> {
        let a = self.pop()?;
        let b = self
            .value_stack
            .last_mut()
            .ok_or(InterpreterError::ValueStackUnderflow)?;

        *b = op(&*b, a)?;

        Ok(())
    }

    fn apply_un_op(
        &mut self,
        op: fn(&Value) -> Result<Value, InterpreterError>,
    ) -> Result<(), InterpreterError> {
        let a = self
            .value_stack
            .last_mut()
            .ok_or(InterpreterError::ValueStackUnderflow)?;

        *a = op(&*a)?;

        Ok(())
    }
}