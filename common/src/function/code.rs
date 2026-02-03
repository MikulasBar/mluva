use bincode::{Decode, Encode};

use crate::instruction::Instruction;

#[derive(Debug, Clone, Encode, Decode)]
pub struct FunctionCode {
    pub slot_count: usize,
    body: Vec<Instruction>,
}

impl FunctionCode {
    pub fn new(slot_count: usize, body: Vec<Instruction>) -> Self {
        Self { slot_count, body }
    }

    pub fn empty() -> Self {
        Self {
            slot_count: 0,
            body: vec![],
        }
    }

    pub fn emit_instr(&mut self, instr: Instruction) {
        self.body.push(instr);
    }

    pub fn last_instr_mut(&mut self) -> Option<&mut Instruction> {
        self.body.last_mut()
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn set_instr_at(&mut self, index: usize, instr: Instruction) {
        self.body[index] = instr;
    }

    pub fn get_instr(&self, index: usize) -> &Instruction {
        &self.body[index]
    }
}
