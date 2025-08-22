use crate::{bytecode::serializable::BytecodeSerializable, compiler::DataType};

pub struct DataTypeId;

impl DataTypeId {
    pub const VOID: u8 = 0;
    pub const BOOL: u8 = 1;
    pub const INT: u8 = 2;
    pub const FLOAT: u8 = 3;
    pub const STRING: u8 = 4;
}

fn get_id(data_type: &DataType) -> u8 {
    match data_type {
        DataType::Void => DataTypeId::VOID,
        DataType::Bool => DataTypeId::BOOL,
        DataType::Int => DataTypeId::INT,
        DataType::Float => DataTypeId::FLOAT,
        DataType::String => DataTypeId::STRING,
    }
}


impl BytecodeSerializable for DataType {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        todo!()
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&get_id(self).to_le_bytes());
    }
}