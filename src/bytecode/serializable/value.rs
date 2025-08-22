use crate::{bytecode::BytecodeSerializable, value::Value};
use super::data_type::DataTypeId;

fn get_id(value: &Value) -> u8 {
    match value {
        Value::Void => DataTypeId::VOID,
        Value::Bool(_) => DataTypeId::BOOL,
        Value::Int(_) => DataTypeId::INT,
        Value::Float(_) => DataTypeId::FLOAT,
        Value::String(_) => DataTypeId::STRING,
    }
}

impl BytecodeSerializable for Value {
    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.push(get_id(self));

        match self {
            Value::Void => (),
            Value::Bool(b) => buffer.push(*b as u8),
            Value::Int(x) => {
                buffer.extend_from_slice(&x.to_le_bytes());
            }
            Value::Float(x) => {
                buffer.extend_from_slice(&x.to_le_bytes());
            }
            Value::String(s) => {
                let len = s.len() as u32; // TODO: handle length exceeding u32::MAX
                buffer.extend_from_slice(&len.to_le_bytes());
                buffer.extend_from_slice(s.as_bytes());
            }
        }
    }

    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + 4 > bytes.len() {
            return Err("Not enough bytes for Value type ID".to_string());
        }

        let type_id = bytes[*cursor];
        *cursor += 1;

        match type_id {
            DataTypeId::VOID => Ok(Value::Void),
            DataTypeId::BOOL => {
                if *cursor >= bytes.len() {
                    return Err("Insufficient bytes for Bool".to_string());
                }
                let b = bytes[*cursor] != 0;
                Ok(Value::Bool(b))
            }
            DataTypeId::INT => {
                if *cursor + 4 > bytes.len() {
                    return Err("Insufficient bytes for Int".to_string());
                }
                let x = i32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                *cursor += 4;
                Ok(Value::Int(x))
            }
            DataTypeId::FLOAT => {
                if *cursor + 8 > bytes.len() {
                    return Err("Insufficient bytes for Float".to_string());
                }
                let x = f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap());
                *cursor += 8;
                Ok(Value::Float(x))
            }
            DataTypeId::STRING => {
                if *cursor + 4 > bytes.len() {
                    return Err("Insufficient bytes for String length".to_string());
                }
                let len = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap()) as usize;
                *cursor += 4;

                if *cursor + len > bytes.len() {
                    return Err("Insufficient bytes for String content".to_string());
                }
                let s = String::from_utf8(bytes[*cursor..*cursor + len].to_vec())
                    .map_err(|e| e.to_string())?;
                *cursor += len;
                Ok(Value::String(s))
            }
            _ => Err(format!("Unknown type identifier: {}", type_id)),
        }
    }
}
