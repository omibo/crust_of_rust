// Explain that UnsafeCell gives a raw pointer.
use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell<T>:
// impl<T> !Sync for Cell<T> {};

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we knoe no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: we knoe we are not invalidating any referneces, because we never give any out.
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we knoe no-one else is modifying this value, since only this thread can mutate (because !Sync),
        // and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}
