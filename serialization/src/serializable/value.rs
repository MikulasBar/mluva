use crate::{bytecode::BytecodeSerializable, reference::FsRef, value::Value};

struct ValueId;
impl ValueId {
    pub const VOID: u8 = 0;
    pub const BOOL: u8 = 1;
    pub const INT: u8 = 2;
    pub const FLOAT: u8 = 3;
    pub const STRING: u8 = 4;
    pub const LIST: u8 = 5;
    pub const FSREF: u8 = 6;
    pub const RCREF: u8 = 7;
}

fn get_id(value: &Value) -> u8 {
    match value {
        Value::Void => ValueId::VOID,
        Value::Bool(_) => ValueId::BOOL,
        Value::Int(_) => ValueId::INT,
        Value::Float(_) => ValueId::FLOAT,
        Value::String(_) => ValueId::STRING,
        Value::List { .. } => ValueId::LIST,
        Value::FsRef(_) => ValueId::FSREF,
        Value::RcRef(_) => ValueId::RCREF,
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
            Value::List(_) => {
                unreachable!("List values shouldn't be serializable")
                // items.len().write_bytecode(buffer);
                // for item in items {
                //     item.write_bytecode(buffer);
                // }
            }
            Value::FsRef(rf) => {
                rf.frame_index.write_bytecode(buffer);
                rf.var_slot.write_bytecode(buffer);
            }
            Value::RcRef(_) => {
                unreachable!("RcRef values shouldn't be serializable")
            }
        }
    }

    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let type_id = u8::from_bytecode(bytes, cursor)?;

        match type_id {
            ValueId::VOID => Ok(Value::Void),
            ValueId::BOOL => {
                let b = bool::from_bytecode(bytes, cursor)?;
                Ok(Value::Bool(b))
            }
            ValueId::INT => {
                let x = i32::from_bytecode(bytes, cursor)?;
                Ok(Value::Int(x))
            }
            ValueId::FLOAT => {
                let x = f64::from_bytecode(bytes, cursor)?;
                Ok(Value::Float(x))
            }
            ValueId::STRING => {
                let s = String::from_bytecode(bytes, cursor)?;
                Ok(Value::String(s))
            }
            ValueId::LIST => {
                unreachable!("List values shouldn't be deserializable")

                // let length = usize::from_bytecode(bytes, cursor)?;
                // let mut items = Vec::with_capacity(length);
                // for _ in 0..length {
                //     let item = Value::from_bytecode(bytes, cursor)?;
                //     items.push(item);
                // }
                // Ok(Value::List(items))
            }
            ValueId::FSREF => {
                let frame_index = usize::from_bytecode(bytes, cursor)?;
                let var_slot = usize::from_bytecode(bytes, cursor)?;
                Ok(Value::FsRef(FsRef::new(frame_index, var_slot)))
            }
            ValueId::RCREF => {
                unreachable!("RcRef values shouldn't be deserializable")
            }
            _ => Err(format!("Unknown type identifier: {}", type_id)),
        }
    }
}
