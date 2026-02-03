use std::{alloc, ptr, sync::Arc};

use crate::{class::ClassEntity, vm::{RuntimeError, ValueStack, object::Object}, word::Word};



pub struct Heap {
    objects: Vec<*mut Object>
}

impl Heap {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    /// based on given class creates object managed by self
    /// 
    /// will also load fields into the object and truncate them from value stack
    pub fn create_object(&mut self, class: Arc<ClassEntity>, value_stack: &mut ValueStack) -> Result<(), RuntimeError> {
        unsafe {
            let body_size = class.object_body_words();
            let obj = Object::new(class);
            let layout = obj.get_full_layout();
            let ptr = alloc::alloc(layout);

            if ptr.is_null() {
                return Err(RuntimeError::other("allocation failed"));
            }

            let src_len = value_stack.len();
            let src = &value_stack.stack[src_len - body_size .. src_len];

            let dst = (ptr as *mut u8).add(size_of::<Object>()) as *mut Word;
            ptr::copy_nonoverlapping(src.as_ptr(), dst, body_size);
            value_stack.stack.truncate(src_len - body_size);
        }

        Ok(())
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        unsafe {
            for p in &self.objects {
                let ptr = *p;
                let layout = (*ptr).get_full_layout();
                alloc::dealloc(ptr as *mut u8, layout);
            }
        }
    }
}