use crate::{bytecode::BytecodeSerializable, value::Value};

struct TypeId;

impl TypeId {
    const VOID: u8 = 0;
    const BOOL: u8 = 1;
    const INT: u8 = 2;
    const FLOAT: u8 = 3;
    const STRING: u8 = 4;
}

fn get_id(value: &Value) -> u8 {
    match value {
        Value::Void => TypeId::VOID,
        Value::Bool(_) => TypeId::BOOL,
        Value::Int(_) => TypeId::INT,
        Value::Float(_) => TypeId::FLOAT,
        Value::String(_) => TypeId::STRING,
    }
}

impl BytecodeSerializable for Value {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.push(get_id(self));

        match self {
            Value::Void => bytes,
            Value::Bool(b) => {
                bytes.push(if *b { 1 } else { 0 });
                bytes
            }
            Value::Int(x) => {
                bytes.extend_from_slice(&x.to_le_bytes());
                bytes
            }
            Value::Float(x) => {
                bytes.extend_from_slice(&x.to_le_bytes());
                bytes
            }
            Value::String(s) => {
                let len = s.len() as u32; // TODO: handle length exceeding u32::MAX
                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(s.as_bytes());
                bytes
            }
        }
    }

    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if bytes.is_empty() {
            return Err("Bytecode is empty".to_string());
        }

        let type_id = bytes[*cursor];
        *cursor += 1;

        match type_id {
            TypeId::VOID => Ok(Value::Void),
            TypeId::BOOL => {
                if *cursor >= bytes.len() {
                    return Err("Insufficient bytes for Bool".to_string());
                }
                let b = bytes[*cursor] != 0;
                Ok(Value::Bool(b))
            }
            TypeId::INT => {
                if *cursor + 4 > bytes.len() {
                    return Err("Insufficient bytes for Int".to_string());
                }
                let x = i32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                *cursor += 4;
                Ok(Value::Int(x))
            }
            TypeId::FLOAT => {
                if *cursor + 8 > bytes.len() {
                    return Err("Insufficient bytes for Float".to_string());
                }
                let x = f64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap());
                *cursor += 8;
                Ok(Value::Float(x))
            }
            TypeId::STRING => {
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
