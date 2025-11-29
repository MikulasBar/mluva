use crate::word::Word;

#[derive(Debug, PartialEq)]
pub struct HeapHandle {
    pub index: u32,
    pub generation: u32,
}

impl HeapHandle {
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    pub fn as_word(&self) -> Word {
        Word::combine(self.index, self.generation)
    }
}
