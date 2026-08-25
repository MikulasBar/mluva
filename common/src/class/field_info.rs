use bincode::{Decode, Encode};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode)]
pub struct FieldInfo {
    is_ref: bool,
}