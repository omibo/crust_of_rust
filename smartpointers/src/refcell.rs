use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(isize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

// implied by UnsafeCell<T>:
// impl<T> !Sync for RefCell<T> {};

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            // If more than one thread have access to one RefCell type, they may both reach to Unshared state at the same time,
            // and both set self.state = RefState::Shared(1).
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: No exclusive references have been given out since state would be exclusive.
                Some(Ref { refcell: self })
            }
            RefState::Shared(num) => {
                self.state.set(RefState::Shared(num + 1));
                // SAFETY: No exclusive references have been given out since state would be exclusive.
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Exclusive);
                // SAFETY: No other references have been given out since state would be Shared or Exclusive.
                Some(RefMut { refcell: self })
            }
            _ => None,
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: A ref is only created if no exclusive references have been given out.
        // Once it is given out, state is set to Shared, so no exclusive references are given out.
        // so derefrencing into a shared reference is fine.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => {
                unreachable!()
            }
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: See Safety for DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: A RefMut is only created if no other references have been given out.
        // Once it is given out, state is set to Exclusive, so no future references are given out.
        // so we have an exclusive lease on the inner value, so mutably derefrencing is fine.
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => {
                unreachable!()
            }
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}
