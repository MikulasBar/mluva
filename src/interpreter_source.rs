use crate::instruction::Instruction;



#[derive(Debug, Clone)]
pub struct InterpreterSource {
    pub instructions: Vec<Instruction>,
    pub local_slots: usize,
}

impl InterpreterSource {
    pub fn new(instructions: Vec<Instruction>, local_slots: usize) -> Self {
        Self {
            instructions,
            local_slots,
        }
    }
}