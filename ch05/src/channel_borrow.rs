use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    /// Create a new channel
    pub const fn new() -> Self {
        Channel {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    /// Create a pair of (Sender, Receiver) that borrow this channel
    pub fn split(&mut self) -> (Sender<'_, T>, Receiver<'_, T>) {
        // The method exclusively borrows self through a mutable reference.
        // This ensure that the caller cannot borrow or move it as long as Sender
        // or Receiver exists.

        // Overwrite *self with a new empty channel to:
        //  1. Make sure that it is in the expected state to create Sender and Receiver
        //  2. Invoke the Drop implementation on the old *self, which will take care
        //     of dropping a message that has been sent but not received.
        //  Q: Then why we need to take an exclusive reference to self as input? Why not just
        //  create a new one when needed?
        *self = Self::new(); // TODO: Why creating a new Channel here?

        (Sender { channel: self }, Receiver { channel: self })
    }
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        }
        self.channel.ready.store(true, Ordering::Release);
    }
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Ordering::Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.load(Ordering::Acquire) {
            panic!("no message available");
        }

        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_can_send_once() {
        let mut channel = Channel::new();
        thread::scope(|s| {
            let (sender, receiver) = channel.split();
            let t = thread::current();

            s.spawn(move || {
                sender.send("Hello, World!");
                t.unpark();
            });

            while !receiver.is_ready() {
                thread::park();
            }

            // Will not compile: Cannot split() when sender and receiver are active.
            // channel.split();

            assert_eq!(receiver.receive(), "Hello, World!");
        });

        // OK to split here
        channel.split();
    }
}
