use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        // If locked is true at the beginning of the function call,
        // `swap` returns true and we retry.
        //
        // If locked is false at the beginning of the function call,
        // `swap` stores true (acquiring the lock), and returns false; the loop stops.
        //
        while self.locked.swap(true, Ordering::Acquire) {
            // Spin loop hint telling the processor that we're spinning while waiting for something
            // to change.
            std::hint::spin_loop();
        }

        // Alternative:
        // while self
        //     .locked
        //     .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
        //     .is_err()
        // {}
        // Note that we can use Ordering::Relaxed for the failure case because we haven't acquired the
        // lock, so there's no need to use Ordering::Acquire to ensure a happens-before relationship
        // with a previous store.
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {}
