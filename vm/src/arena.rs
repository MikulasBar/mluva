use std::{
    alloc::{self, Layout},
    ptr::NonNull,
};

use crate::{
    runtime_error::RuntimeError,
    value_stack::ValueStack,
    vtable::{Method, VTable},
};

use common::{heap_handle::HeapHandle, type_manager::PRIMITIVE_TYPES_COUNT};

pub struct Arena {
    slots: Vec<Slot>,
    free_head: Option<u32>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_head: None,
        }
    }

    pub fn get<T>(&self, handle: &HeapHandle) -> Result<&T, RuntimeError> {
        if let Some(Slot {
            generation,
            slot_data: SlotData::Occupied { object, .. },
        }) = self.slots.get(handle.index as usize)
        {
            if *generation == handle.generation {
                let ptr = object.as_ptr() as *const T;
                return Ok(unsafe { &*ptr });
            }
        }
        Err(RuntimeError::InvalidHeapHandle)
    }

    pub fn get_mut<T>(&mut self, handle: &HeapHandle) -> Result<&mut T, RuntimeError> {
        if let Some(Slot {
            generation,
            slot_data: SlotData::Occupied { object, .. },
        }) = self.slots.get_mut(handle.index as usize)
        {
            if *generation == handle.generation {
                let ptr = object.as_ptr() as *mut T;
                return Ok(unsafe { &mut *ptr });
            }
        }
        Err(RuntimeError::InvalidHeapHandle)
    }

    pub fn alloc<T>(&mut self, type_id: u32, object: T) -> HeapHandle {
        let layout = Layout::new::<T>();
        let object = NonNull::new(Box::into_raw(Box::new(object)) as *mut u8).unwrap();

        let index = if let Some(idx) = self.free_head {
            let slot = &mut self.slots[idx as usize];
            if let Slot {
                generation,
                slot_data: SlotData::Free { next },
            } = slot
            {
                self.free_head = *next;
                *slot = Slot::new_occupied(*generation + 1, type_id, layout, object);
            }
            idx
        } else {
            let idx = self.slots.len() as u32;
            self.slots
                .push(Slot::new_occupied(0, type_id, layout, object));
            idx
        };
        HeapHandle::new(index, 0)
    }

    pub fn increment_rc(&mut self, handle: &HeapHandle) -> Result<(), RuntimeError> {
        if let Some(Slot {
            generation,
            slot_data: SlotData::Occupied { refcount, .. },
        }) = self.slots.get_mut(handle.index as usize)
        {
            if *generation == handle.generation {
                *refcount += 1;
                return Ok(());
            }
        }
        Err(RuntimeError::InvalidHeapHandle)
    }

    pub fn decrement_rc(
        &mut self,
        handle: &HeapHandle,
        value_stack: &mut ValueStack,
        vtables: &[VTable],
    ) -> Result<(), RuntimeError> {
        if let Some(Slot {
            generation,
            slot_data:
                SlotData::Occupied {
                    refcount,
                    type_id,
                    layout,
                    object,
                },
        }) = self.slots.get_mut(handle.index as usize)
        {
            let layout = *layout;
            let object = *object;
            if *generation == handle.generation {
                if *refcount == 0 {
                    return Err(RuntimeError::InvalidHeapHandle);
                }
                *refcount -= 1;

                if *refcount != 0 {
                    return Ok(());
                }

                if *type_id >= PRIMITIVE_TYPES_COUNT {
                    let vtable = &vtables[*type_id as usize];
                    if let Some(Method::Native { func: destr }) =
                        vtable.methods.get(VTable::DESTRUCTOR_SLOT)
                    {
                        destr(handle.as_word(), value_stack, self, vtables);
                        unsafe {
                            alloc::dealloc(object.as_ptr(), layout);
                        }

                        let slot = &mut self.slots[handle.index as usize];
                        let next_free = self.free_head;
                        slot.generation += 1;
                        slot.slot_data = SlotData::Free { next: next_free };
                        self.free_head = Some(handle.index);
                    }
                }

                return Ok(());
            }
        }

        Err(RuntimeError::InvalidHeapHandle)
    }
}

struct Slot {
    generation: u32,
    slot_data: SlotData,
}

impl Slot {
    pub fn new_occupied(
        generation: u32,
        type_id: u32,
        layout: Layout,
        object: NonNull<u8>,
    ) -> Self {
        Self {
            generation,
            slot_data: SlotData::new_occupied(type_id, layout, object),
        }
    }

    pub fn next_free(&self) -> Option<u32> {
        if let SlotData::Free { next } = &self.slot_data {
            *next
        } else {
            None
        }
    }
}

enum SlotData {
    Free {
        next: Option<u32>,
    },
    Occupied {
        refcount: u32,
        type_id: u32,
        layout: Layout,
        object: NonNull<u8>,
    },
}

impl SlotData {
    pub fn new_occupied(type_id: u32, layout: Layout, object: NonNull<u8>) -> Self {
        Self::Occupied {
            refcount: 1,
            type_id,
            layout,
            object,
        }
    }
}
