use crate::{bytecode::serializable::BytecodeSerializable, function::{InternalFunctionDefinition, InternalFunctionSource}, instruction::Instruction};



impl BytecodeSerializable for InternalFunctionDefinition {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        todo!()
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        self.return_type.write_bytecode(buffer);

        let param_count = self.params.len() as u32;
        buffer.extend_from_slice(&param_count.to_le_bytes());

        for (name, datatype) in &self.params {
            buffer.extend_from_slice(name.as_bytes());
            buffer.push(0); // Null terminator
            datatype.write_bytecode(buffer);
        }
    }
}


impl BytecodeSerializable for InternalFunctionSource {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if bytes.len() < *cursor + 4 {
            return Err("Not enough bytes for slot count".to_string());
        }
        let slot_count = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as usize;
        *cursor += 4;

        if bytes.len() < *cursor + 4 {
            return Err("Not enough bytes for instruction count".to_string());
        }
        let instr_count = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as usize;
        *cursor += 4;

        let mut body = Vec::with_capacity(instr_count);
        for _ in 0..instr_count {
            let instruction = Instruction::from_bytecode(bytes, cursor)?;
            body.push(instruction);
        }

        Ok(InternalFunctionSource {
            slot_count,
            body,
        })
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        let slot_count = self.slot_count as u32;
        buffer.extend_from_slice(&slot_count.to_le_bytes());

        let instr_count = self.body.len() as u32;
        buffer.extend_from_slice(&instr_count.to_le_bytes());

        for instruction in &self.body {
            instruction.write_bytecode(buffer);
        }
    }
}