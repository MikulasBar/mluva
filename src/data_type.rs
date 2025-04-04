
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
}


impl DataType {
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }
}