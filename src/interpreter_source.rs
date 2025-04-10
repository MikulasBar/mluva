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

    // pub fn to_bytecode(&self) -> Vec<u8> {
    //     let mut bytecode = vec![];


    //     let x = 45456 as usize;

    //     let y = x.to_le_bytes();

    //     bytecode.push(self.local_slots as u8); // Local slots count

    //     for instruction in &self.instructions {
    //         match instruction {
    //             Instruction::Push(value) => {
    //                 bytecode.push(0x01); // Push opcode
    //                 bytecode.extend_from_slice(&value.to_le_bytes());
    //             }
    //         }
    //     }
    // }
}