mod data_type;
mod function;
mod instruction;
mod value;

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

impl BytecodeSerializable for usize {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let value = u32::from_bytecode(bytes, cursor)? as usize;
        Ok(value)
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        if self > &(u32::MAX as usize) {
            panic!("usize value exceeds u32::MAX"); // TODO: handle this more gracefully
        }
        let value = *self as u32;
        buffer.extend_from_slice(&value.to_le_bytes());
    }
}

impl BytecodeSerializable for String {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let len = usize::from_bytecode(bytes, cursor)?;

        if *cursor + len > bytes.len() {
            return Err("Not enough bytes for string data".to_string());
        }
        let s = String::from_utf8(bytes[*cursor..*cursor + len].to_vec())
            .map_err(|e| format!("Invalid UTF-8 string: {}", e))?;
        *cursor += len;
        Ok(s)
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        self.len().write_bytecode(buffer);
        buffer.extend_from_slice(self.as_bytes());
    }
}

impl BytecodeSerializable for bool {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let byte = u8::from_bytecode(bytes, cursor)?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(format!("Invalid boolean value: {}", byte)),
        }
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.push(*self as u8);
    }
}

impl BytecodeSerializable for i32 {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + 4 > bytes.len() {
            return Err("Unexpected end of bytecode".to_string());
        }
        let value = i32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        Ok(value)
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_le_bytes());
    }
}

impl BytecodeSerializable for f64 {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + 8 > bytes.len() {
            return Err("Unexpected end of bytecode".to_string());
        }
        let value = f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap());
        *cursor += 8;
        Ok(value)
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_le_bytes());
    }
}
