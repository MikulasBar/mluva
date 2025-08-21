use crate::{bytecode::serializable::BytecodeSerializable, compiler::DataType};

pub struct DataTypeId;

impl DataTypeId {
    pub const VOID: u32 = 0;
    pub const BOOL: u32 = 1;
    pub const INT: u32 = 2;
    pub const FLOAT: u32 = 3;
    pub const STRING: u32 = 4;
}

fn get_id(data_type: &DataType) -> u32 {
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