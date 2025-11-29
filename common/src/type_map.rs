use std::collections::HashMap;

pub const PRIMITIVE_TYPES_COUNT: u32 = 3;
pub const I32_TYPE_ID: u32 = 0;
pub const F32_TYPE_ID: u32 = 1;
pub const BOOL_TYPE_ID: u32 = 2;
pub const STRING_TYPE_ID: u32 = 4;
pub const LIST_TYPE_ID: u32 = 5;

pub struct TypeManager {
    type_slots: HashMap<String, u32>,
    types: Vec<Type>,
}

pub struct Type {
    pub id: u32,
    pub name: String,
    methods: HashMap<String, u32>,
}
