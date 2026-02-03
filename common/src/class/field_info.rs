use bincode::{Decode, Encode};


#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct FieldInfo {
    is_ref: bool,
}