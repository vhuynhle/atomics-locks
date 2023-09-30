use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::{cell::UnsafeCell, sync::atomic::AtomicBool, thread};

use rand::{thread_rng, Rng};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

/// This struct ties the unlocking operation to the end of &mut T.
/// It does that by:
/// - wrapping the reference &mut T in our own type
/// - behave like a reference (impl Deref and DerefMut)
/// - When it is dropped, unlock the spin lock
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: The existence of this lock guarantees that we have
        // exclusively locked the lock
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: The existence of this lock guarantees that we have exclusively
        // locked the lock
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        // The Guard type does not have a constructor, and its field (lock) is
        // private, so this is the only way a Guard object can be created.
        Guard { lock: self }
    }
}

fn main() {
    let mut thread_1_wins = 0;
    let mut thread_2_wins = 0;
    for _ in 1..10000 {
        let x = SpinLock::new(Vec::new());
        thread::scope(|s| {
            s.spawn(|| {
                let duration = Duration::from_micros(thread_rng().gen_range(1..=3));
                thread::sleep(duration);
                x.lock().push(1);
            });
            s.spawn(|| {
                let duration = Duration::from_micros(thread_rng().gen_range(1..=3));
                thread::sleep(duration);
                let mut g = x.lock();
                g.push(2); // call deref_mut implicitly
                g.deref_mut().push(3); // Longer way to do it
            });
        });
        let g = x.lock();
        if g.as_slice() == [1, 2, 3] {
            thread_1_wins += 1;
        } else if g.as_slice() == [2, 3, 1] {
            thread_2_wins += 1;
        } else {
            panic!("Our Guard is not working!");
        }
    }

    println!(
        "Thread 1 wins {} time(s), Thread 2 wins {} time(s)",
        thread_1_wins, thread_2_wins
    );
}
