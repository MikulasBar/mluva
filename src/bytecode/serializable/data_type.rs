use crate::{bytecode::serializable::BytecodeSerializable, data_type::DataType};

pub struct DataTypeId;

impl DataTypeId {
    pub const VOID: u8 = 0;
    pub const BOOL: u8 = 1;
    pub const INT: u8 = 2;
    pub const FLOAT: u8 = 3;
    pub const STRING: u8 = 4;
    pub const LIST: u8 = 5;
}

fn get_id(data_type: &DataType) -> u8 {
    match data_type {
        DataType::Void => DataTypeId::VOID,
        DataType::Bool => DataTypeId::BOOL,
        DataType::Int => DataTypeId::INT,
        DataType::Float => DataTypeId::FLOAT,
        DataType::String => DataTypeId::STRING,
        DataType::List { .. } => DataTypeId::LIST,
    }
}

impl BytecodeSerializable for DataType {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let id = u8::from_bytecode(bytes, cursor)?;
        match id {
            DataTypeId::VOID => Ok(DataType::Void),
            DataTypeId::BOOL => Ok(DataType::Bool),
            DataTypeId::INT => Ok(DataType::Int),
            DataTypeId::FLOAT => Ok(DataType::Float),
            DataTypeId::STRING => Ok(DataType::String),
            DataTypeId::LIST => {
                let item_type = Box::new(DataType::from_bytecode(bytes, cursor)?);
                Ok(DataType::List { item_type })
            }

            _ => Err(format!("Unknown DataType id: {}", id)),
        }
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        get_id(self).write_bytecode(buffer);
    }
}
