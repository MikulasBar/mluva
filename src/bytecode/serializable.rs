

mod value;
mod instruction;
mod function;
mod data_type;


pub trait BytecodeSerializable: Sized {
    fn write_bytecode(&self, buffer: &mut Vec<u8>);
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String>;
}


impl BytecodeSerializable for u8 {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + 1 > bytes.len() {
            return Err("Unexpected end of bytecode".to_string());
        }
        let value = bytes[*cursor];
        *cursor += 1;
        Ok(value)
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.push(*self);
    }
}


impl<T: BytecodeSerializable> BytecodeSerializable for Option<T> {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let is_some = u8::from_bytecode(bytes, cursor)?;
        if is_some != 0 {
            let value = T::from_bytecode(bytes, cursor)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        match self {
            Some(value) => {
                buffer.push(1);
                value.write_bytecode(buffer);
            }
            None => {
                buffer.push(0);
            }
        }
    }
}

impl BytecodeSerializable for u32 {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + 4 > bytes.len() {
            return Err("Unexpected end of bytecode".to_string());
        }
        let value = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        Ok(value)
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_le_bytes());
    }
}