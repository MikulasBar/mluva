mod value;
mod instruction;


const MAGIC: &[u8] = b"\0mluvabc";
const VERSION: u8 = 0;

pub trait BytecodeSerializable: Sized {
    fn to_bytecode(&self) -> Vec<u8>;
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String>;
}