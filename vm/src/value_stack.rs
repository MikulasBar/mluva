use crate::{runtime_error::RuntimeError, word::Word};

pub struct ValueStack {
    stack: Vec<Word>,
}

impl ValueStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: Word) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Word, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::ValueStackUnderflow)
    }

    pub fn last_mut(&mut self) -> Result<&mut Word, RuntimeError> {
        self.stack
            .last_mut()
            .ok_or(RuntimeError::ValueStackUnderflow)
    }

    pub fn copy_last(&self) -> Result<Word, RuntimeError> {
        self.stack
            .last()
            .copied()
            .ok_or(RuntimeError::ValueStackUnderflow)
    }

    pub fn split_off(&mut self, at: usize) -> Vec<Word> {
        self.stack.split_off(at)
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}
