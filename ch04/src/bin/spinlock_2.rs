use std::{cell::UnsafeCell, sync::atomic::AtomicBool};
use std::sync::atomic::Ordering;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T>
    where T: Send
{}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        unsafe {
            &mut *self.value.get()
        }
    }

    /// # Safety
    ///
    /// The &mut T from lock() must be gone before this function
    /// can be called.
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {}
