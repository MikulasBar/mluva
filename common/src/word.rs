use crate::heap_handle::HeapHandle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Word {
    pub data: u64,
}

impl Word {
    pub fn new(data: u64) -> Self {
        Self { data }
    }

    pub fn void() -> Self {
        Self { data: 0 }
    }

    pub fn split(&self) -> (u32, u32) {
        (self.data as u32, (self.data >> 32) as u32)
    }

    pub fn combine(lhs: u32, rhs: u32) -> Self {
        Self {
            data: (lhs as u64) | ((rhs as u64) << 32),
        }
    }

    #[inline(always)]
    pub fn as_u32(&self) -> u32 {
        self.data as u32
    }

    #[inline(always)]
    pub fn as_i32(&self) -> i32 {
        self.data as i32
    }

    #[inline(always)]
    pub fn as_f32(&self) -> f32 {
        f32::from_bits(self.data as u32)
    }

    #[inline(always)]
    pub fn as_bool(&self) -> bool {
        self.data != 0
    }

    #[inline(always)]
    pub fn as_hhandle(&self) -> HeapHandle {
        let (index, generation) = self.split();
        HeapHandle::new(index, generation)
    }

    #[inline(always)]
    pub fn from_i32(value: i32) -> Self {
        Self { data: value as u64 }
    }

    #[inline(always)]
    pub fn from_f32(value: f32) -> Self {
        Self {
            data: value.to_bits() as u64,
        }
    }

    #[inline(always)]
    pub fn from_bool(value: bool) -> Self {
        Self {
            data: if value { 1 } else { 0 },
        }
    }

    #[inline(always)]
    pub fn cmp_assign_equal(&mut self, other: Self) {
        self.data = if self.data == other.data { 1 } else { 0 };
    }

    #[inline(always)]
    pub fn cmp_assign_not_equal(&mut self, other: Self) {
        self.data = if self.data != other.data { 1 } else { 0 };
    }

    #[inline(always)]
    pub fn bool_assign_and(&mut self, other: Self) {
        self.data &= other.data;
    }

    #[inline(always)]
    pub fn bool_assign_or(&mut self, other: Self) {
        self.data |= other.data;
    }

    #[inline(always)]
    pub fn bool_assign_not(&mut self) {
        self.data = !self.data;
    }

    #[inline(always)]
    pub fn i32_assign_add(&mut self, other: Self) {
        self.data = self.as_i32().wrapping_add(other.as_i32()) as u64;
    }

    #[inline(always)]
    pub fn i32_assign_sub(&mut self, other: Self) {
        self.data = self.as_i32().wrapping_sub(other.as_i32()) as u64;
    }

    #[inline(always)]
    pub fn i32_assign_mul(&mut self, other: Self) {
        self.data = self.as_i32().wrapping_mul(other.as_i32()) as u64;
    }

    #[inline(always)]
    pub fn i32_assign_div(&mut self, other: Self) {
        self.data = self.as_i32().wrapping_div(other.as_i32()) as u64;
    }

    #[inline(always)]
    pub fn i32_assign_modulo(&mut self, other: Self) {
        self.data = self.as_i32().wrapping_rem(other.as_i32()) as u64;
    }

    #[inline(always)]
    pub fn i32_assign_negate(&mut self) {
        self.data = self.as_i32().wrapping_neg() as u64;
    }

    #[inline(always)]
    pub fn i32_assign_greater(&mut self, other: Self) {
        self.data = if self.as_i32() > other.as_i32() { 1 } else { 0 };
    }

    #[inline(always)]
    pub fn i32_assign_greater_equal(&mut self, other: Self) {
        self.data = if self.as_i32() >= other.as_i32() {
            1
        } else {
            0
        };
    }

    #[inline(always)]
    pub fn i32_assign_less(&mut self, other: Self) {
        self.data = if self.as_i32() < other.as_i32() { 1 } else { 0 };
    }

    #[inline(always)]
    pub fn i32_assign_less_equal(&mut self, other: Self) {
        self.data = if self.as_i32() <= other.as_i32() {
            1
        } else {
            0
        };
    }

    #[inline(always)]
    pub fn f32_assign_add(&mut self, other: Self) {
        self.data = (self.as_f32() + other.as_f32()).to_bits() as u64;
    }

    #[inline(always)]
    pub fn f32_assign_sub(&mut self, other: Self) {
        self.data = (self.as_f32() - other.as_f32()).to_bits() as u64;
    }

    #[inline(always)]
    pub fn f32_assign_mul(&mut self, other: Self) {
        self.data = (self.as_f32() * other.as_f32()).to_bits() as u64;
    }

    #[inline(always)]
    pub fn f32_assign_div(&mut self, other: Self) {
        self.data = (self.as_f32() / other.as_f32()).to_bits() as u64;
    }

    #[inline(always)]
    pub fn f32_assign_modulo(&mut self, other: Self) {
        self.data = (self.as_f32() % other.as_f32()).to_bits() as u64;
    }

    #[inline(always)]
    pub fn f32_assign_negate(&mut self) {
        self.data = (-self.as_f32()).to_bits() as u64;
    }

    #[inline(always)]
    pub fn f32_assign_greater(&mut self, other: Self) {
        self.data = if self.as_f32() > other.as_f32() { 1 } else { 0 };
    }

    #[inline(always)]
    pub fn f32_assign_greater_equal(&mut self, other: Self) {
        self.data = if self.as_f32() >= other.as_f32() {
            1
        } else {
            0
        };
    }

    #[inline(always)]
    pub fn f32_assign_less(&mut self, other: Self) {
        self.data = if self.as_f32() < other.as_f32() { 1 } else { 0 };
    }

    #[inline(always)]
    pub fn f32_assign_less_equal(&mut self, other: Self) {
        self.data = if self.as_f32() <= other.as_f32() {
            1
        } else {
            0
        };
    }
}
