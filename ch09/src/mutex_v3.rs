use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

use atomic_wait::{wait, wake_one};

pub struct Mutex<T> {
    /// 0: unlocked,
    /// 1: locked, no other threads waiting
    /// 2: locked, other threads waiting
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0), // unlocked at creation
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        if self
            .state
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            lock_contended(&self.state); // Now we have the lock and state = 2.
        } else {
            // Success: The state was 0 before. We have now acquired the lock and set
            // state to 1.
        }

        MutexGuard { mutex: self }
    }
}

fn lock_contended(state: &AtomicU32) {
    // spin for a short time, in case the contension is low
    let mut spin_count = 0;
    while state.load(Ordering::Relaxed) == 1 && spin_count < 100 {
        spin_count += 1;
        std::hint::spin_loop();
    }

    // Check after spinning, in case the lock is free now
    if state
        .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        return;
    }

    // Continue waiting using the system call
    while state.swap(2, Ordering::Acquire) != 0 {
        wait(state, 2);
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // Swap with the current state (1 or 2) with 0
        // to release the lock.
        if self.mutex.state.swap(0, Ordering::Release) == 2 {
            // Previous state was 2: Some other threads are waiting for the lock
            // Only in this case we call wake_one.
            wake_one(&self.mutex.state);

            // The state has been set to 0, so any other waiting thread should
            // set the state back to 2 after waiting the lock in order not to forget other threads.
            // See (1)
        }
    }
}
