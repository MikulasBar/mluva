use crate::{
    bytecode::serializable::BytecodeSerializable, compiler::DataType, function::{InternalFunctionSigniture, InternalFunctionSource}, instruction::Instruction
};

impl BytecodeSerializable for InternalFunctionSigniture {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let return_type = DataType::from_bytecode(bytes, cursor)?;
        let param_count = usize::from_bytecode(bytes, cursor)?;

        let mut params = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            let name = String::from_bytecode(bytes, cursor)?;
            let datatype = DataType::from_bytecode(bytes, cursor)?;
            params.push((name, datatype));
        }

        Ok(InternalFunctionSigniture { return_type, params })        
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        self.return_type.write_bytecode(buffer);
        self.params.len().write_bytecode(buffer);

        for (name, datatype) in &self.params {
            name.write_bytecode(buffer);
            datatype.write_bytecode(buffer);
        }
    }
}

impl BytecodeSerializable for InternalFunctionSource {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let slot_count = usize::from_bytecode(bytes, cursor)?;
        let instr_count = usize::from_bytecode(bytes, cursor)?;

        let mut body = Vec::with_capacity(instr_count);
        for _ in 0..instr_count {
            let instruction = Instruction::from_bytecode(bytes, cursor)?;
            body.push(instruction);
        }

        Ok(InternalFunctionSource { slot_count, body })
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        self.slot_count.write_bytecode(buffer);
        self.body.len().write_bytecode(buffer);

        for instruction in &self.body {
            instruction.write_bytecode(buffer);
        }
    }
}
