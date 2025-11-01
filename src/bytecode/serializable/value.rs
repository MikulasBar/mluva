use super::data_type::DataTypeId;
use crate::{bytecode::BytecodeSerializable, value::Value};

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
            Value::Bool(b) => b.write_bytecode(buffer),
            Value::Int(x) => x.write_bytecode(buffer),
            Value::Float(x) => x.write_bytecode(buffer),
            Value::String(s) => s.write_bytecode(buffer),
        }
    }

    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let type_id = u8::from_bytecode(bytes, cursor)?;

        match type_id {
            DataTypeId::VOID => Ok(Value::Void),
            DataTypeId::BOOL => {
                let b = bool::from_bytecode(bytes, cursor)?;
                Ok(Value::Bool(b))
            }
            DataTypeId::INT => {
                let x = i32::from_bytecode(bytes, cursor)?;
                Ok(Value::Int(x))
            }
            DataTypeId::FLOAT => {
                let x = f64::from_bytecode(bytes, cursor)?;
                Ok(Value::Float(x))
            }
            DataTypeId::STRING => {
                let s = String::from_bytecode(bytes, cursor)?;
                Ok(Value::String(s))
            }
            _ => Err(format!("Unknown type identifier: {}", type_id)),
        }
    }
}
