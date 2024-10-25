use crate::cell::Cell;
use std::cell::UnsafeCell;

/// Internal state representing the borrowing state of `RefCell`.
/// - `Unshared`: No references currently borrowed.
/// - `Shared(isize)`: Shared references are borrowed, with `isize` tracking count.
/// - `Exclusive`: A mutable, exclusive reference is borrowed.
#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(isize),
    Exclusive,
}

/// `RefCell` provides interior mutability, allowing controlled mutable or shared access.
/// This implementation enforces Rust's borrowing rules at runtime.
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

// Explicitly implies that `RefCell<T>` is not `Sync` due to interior mutability.
impl<T> RefCell<T> {
    /// Creates a new `RefCell` with an initial value.
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    /// Attempts to borrow an immutable reference, returning `Some` if successful,
    /// or `None` if an exclusive reference already exists.
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            // In a multithreaded context, two threads could reach this point
            // simultaneously, both setting state to `Shared(1)`.
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // No mutable references exist; exclusive access would set state to `Exclusive`.
                Some(Ref { refcell: self })
            }
            RefState::Shared(num) => {
                self.state.set(RefState::Shared(num + 1));
                // Only shared references exist; exclusive access would set state to `Exclusive`.
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None, // Exclusive reference exists; no shared access allowed.
        }
    }

    /// Attempts to borrow a mutable reference, returning `Some` if successful,
    /// or `None` if any other references (shared or exclusive) exist.
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Exclusive);
                // No other references exist; safe to allow exclusive access.
                Some(RefMut { refcell: self })
            }
            _ => None, // Shared or exclusive references exist; no mutable access allowed.
        }
    }
}

/// A shared reference to the value inside a `RefCell`.
/// Borrowed only when no mutable references exist.
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: `Ref` is only created if no exclusive references exist.
        // State is set to `Shared`, preventing future exclusive borrows.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => {
                unreachable!() // Invalid state; `Ref` would not exist if these were set.
            }
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared); // Last shared reference dropped.
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1)); // Decrement shared count.
            }
        }
    }
}

/// An exclusive, mutable reference to the value inside a `RefCell`.
/// Borrowed only when no other references exist.
pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: Refer to the `DerefMut` implementation for reasoning.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: `RefMut` is only created if no other references exist.
        // State is set to `Exclusive`, preventing any future borrows.
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => {
                unreachable!() // Invalid state; `RefMut` would not exist if these were set.
            }
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared); // Exclusive access ended.
            }
        }
    }
}
