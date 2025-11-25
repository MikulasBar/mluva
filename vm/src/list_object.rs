use crate::{runtime_error::RuntimeError, word::Word};

pub struct ListObject {
    items: Vec<Word>,
    item_type_id: u32,
}

impl ListObject {
    pub fn from_values(item_type_id: u32, values: Vec<Word>) -> Self {
        Self {
            items: values,
            item_type_id,
        }
    }

    pub fn get_item(&self, index: u32) -> Result<Word, RuntimeError> {
        self.items
            .get(index as usize)
            .copied()
            .ok_or(RuntimeError::index_out_of_bounds(
                index as i32,
                self.items.len() as u32,
            ))
    }

    pub fn set_item(&mut self, index: u32, value: Word) -> Result<(), RuntimeError> {
        let len = self.items.len() as u32;
        self.items
            .get_mut(index as usize)
            .ok_or(RuntimeError::index_out_of_bounds(index as i32, len))?
            .clone_from(&value);

        Ok(())
    }
}
