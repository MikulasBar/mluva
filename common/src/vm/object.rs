use std::{alloc::Layout, mem, sync::Arc};

use crate::class::ClassEntity;

/// Represents object in VM
/// 
/// doesn't contain the body because its dynamic
#[repr(C)]
pub struct Object {
    pub class: Arc<ClassEntity>,
    pub gc_mark: bool,
}

impl Object {
    pub fn new(class: Arc<ClassEntity>) -> Self {
        Self { class, gc_mark: false }
    }

    pub fn get_full_layout(&self) -> Layout {
        let header_size = mem::size_of::<Object>();
        let body_size = self.class.object_body_size();
        let align = mem::align_of::<Object>();
        let size = header_size + body_size;

        Layout::from_size_align(size, align).unwrap()
    }
}