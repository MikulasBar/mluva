#[derive(Debug, Clone, Copy)]
pub struct LocalSlot {
    pub index: u32,
}

impl LocalSlot {
    pub fn new(index: u32) -> Self {
        Self { index }
    }
}
