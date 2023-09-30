use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Channel<T> {
    /// Storage
    // Use MaybeUninit instead of Option to save memory.
    // MaybeUninit is a bare-bone version of Option; it requires user to
    // manually track its state. In this case, we will use `ready` for that purpose.
    message: UnsafeCell<MaybeUninit<T>>,

    /// State
    ready: AtomicBool,
}

// Tell the compiler that Channel is safe to share across threads,
// as long as T is such.
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl <T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false)
        }
    }

    /// # Safety
    /// Only call this once!
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);

        // The store effectively releases the message to the receiver,
        // so we need to use the release ordering.
        self.ready.store(true, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// # Safety
    /// Only call this after is_ready() returns true!
    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}
