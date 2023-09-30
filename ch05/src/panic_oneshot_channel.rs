use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Channel<T> {
    /// Storage
    // Use MaybeUninit instead of Option to save memory.
    // MaybeUninit is a bare-bone version of Option; it requires user to
    // manually track its state. In this case, we will use `ready` for that purpose.
    message: UnsafeCell<MaybeUninit<T>>,

    /// State: Whether send() has finished
    ready: AtomicBool,

    /// State: Whether send() has already started
    in_use: AtomicBool,
}

// Tell the compiler that Channel is safe to share across threads,
// as long as T is such.
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }

    /// # Safety
    /// Only call this once!
    pub fn send(&self, message: T) {
        // Due to the total modification order of in_use,
        // Relaxed ordering is sufficient here
        if self.in_use.swap(true, Ordering::Relaxed) {
            panic!("Can't send more than one message!");
        }
        unsafe { (*self.message.get()).write(message) };

        // The store effectively releases the message to the receiver,
        // so we need to use the release ordering.
        self.ready.store(true, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        // 1. The ready.load(Acquire) in receive provides the necessary synchronization
        // so we can lower the memory ordering of the load below.
        // 2. The total modification order on `ready` guarantees that after `is_ready` loads
        // `true`, `receive` will also see `true`. So as long as the user of this struct
        // check `is_ready` before receive, the receive is safe.
        self.ready.load(Ordering::Relaxed)
    }

    /// # Safety
    /// Only call this after is_ready() returns true!
    pub fn receive(&self) -> T {
        // Check and reset the ready flag
        if !self.ready.swap(false, Ordering::Acquire) {
            // Panic to show a clear error to the user.
            panic!("No message available");
        }

        // Safety: We have checked and reset the ready flag.
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        // Here we don't need to use an atomic operation, because
        // `drop` can only be called by a thread fully owning the channel.
        // The `get_mut` method takes an exclusive reference (&mut self), confirming that atomic access
        // is not necessary.
        // The same argument holds for self.message
        if *self.ready.get_mut() {
            // The message has been sent but not received.
            // Drop it manually
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::Channel;

    #[test]
    fn can_send_and_receive_one_message() {
        let c = Channel::<i32>::new();
        let t = thread::current();
        thread::scope(|s| {
            s.spawn(|| {
                c.send(42);
                t.unpark();
            });

            while !c.is_ready() {
                thread::park();
            }

            assert_eq!(c.receive(), 42);
        })
    }

    #[test]
    #[should_panic]
    fn panic_if_is_ready_is_not_checked() {
        let c = Channel::<i32>::new();
        let _ = c.receive();
    }

    #[test]
    #[should_panic]
    fn panic_if_receive_more_than_once() {
        let c = Channel::<i32>::new();
        c.send(1000);
        c.receive();
        c.receive();
    }
}
