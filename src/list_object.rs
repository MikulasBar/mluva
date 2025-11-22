use std::{
    alloc::{alloc, Layout},
    ptr::NonNull,
};

use crate::{errors::RuntimeError, word::Word};

pub struct ListObject {
    raw: NonNull<Word>,
    item_type_id: u32,
    length: u32,
    capacity: u32,
}

impl ListObject {
    pub fn from_values(item_type_id: u32, values: Vec<Word>) -> Self {
        let length = values.len() as u32;
        let capacity = length;
        let layout = Layout::array::<Word>(values.len()).unwrap();
        let raw_ptr = unsafe { alloc(layout) } as *mut Word;

        for (i, word) in values.into_iter().enumerate() {
            unsafe {
                raw_ptr.add(i).write(word);
            }
        }

        Self {
            raw: NonNull::new(raw_ptr).unwrap(),
            item_type_id,
            length,
            capacity,
        }
    }

    pub fn get_item(&self, index: u32) -> Result<Word, RuntimeError> {
        if index >= self.length {
            return Err(RuntimeError::index_out_of_bounds(index as i32, self.length));
        }

        unsafe {
            let word_ptr = self.raw.add(index as usize);
            Ok(word_ptr.read())
        }
    }

    pub fn set_item(&mut self, index: u32, value: Word) -> Result<(), RuntimeError> {
        if index >= self.length {
            return Err(RuntimeError::index_out_of_bounds(index as i32, self.length));
        }

        unsafe {
            let word_ptr = self.raw.add(index as usize);
            word_ptr.write(value);
        }

        Ok(())
    }
}
