#[derive(Debug, Clone, Copy)]
pub struct LocalSlot {
    pub index: u32,
    pub type_id: u32,
}

impl LocalSlot {
    pub fn new(index: u32, type_id: u32) -> Self {
        Self { index, type_id }
    }
}
