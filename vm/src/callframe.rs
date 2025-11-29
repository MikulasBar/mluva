use common::word::Word;

pub struct CallFrame {
    pub locals: Vec<Word>,
}

impl CallFrame {
    pub fn new(local_count: u32) -> Self {
        Self {
            locals: vec![Word::void(); local_count as usize],
        }
    }
}
