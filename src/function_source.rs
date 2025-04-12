use crate::instruction::Instruction;



#[derive(Debug, Clone)]
pub struct FunctionSource {
    pub slot_count: usize,
    pub body: Vec<Instruction>,
}

impl FunctionSource {
    pub fn new(slot_count: usize, body: Vec<Instruction>) -> Self {
        Self {
            slot_count,
            body,
        }
    }
}