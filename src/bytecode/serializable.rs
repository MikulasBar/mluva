

mod value;
mod instruction;
mod function;
mod data_type;


pub trait BytecodeSerializable: Sized {
    fn write_bytecode(&self, buffer: &mut Vec<u8>);
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String>;
}
