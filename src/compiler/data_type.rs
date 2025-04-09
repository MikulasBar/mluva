
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Void,
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