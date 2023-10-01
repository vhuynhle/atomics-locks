use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
    thread::{self, Thread},
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiveing_thread: Thread, // The thread to unpark when data is ready
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    // Prevent Receiver from being sent over threads
    // This ensure that the sender can unpark the correct thread
    _no_send: PhantomData<*const ()>, // Use a raw pointer type `*const ()`, which does not implement Send
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

        (
            Sender {
                channel: self,
                receiveing_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            },
        )
    }
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        }
        self.channel.ready.store(true, Ordering::Release);
        self.receiveing_thread.unpark();
    }
}

impl<T> Receiver<'_, T> {
    pub fn receive(self) -> T {
        // Block until the channel is ready
        while !self.channel.ready.swap(false, Ordering::Acquire) {
            thread::park();
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
    use std::{thread, time::Duration};

    #[test]
    fn test_can_send_once() {
        let mut channel = Channel::new();
        thread::scope(|s| {
            let (sender, receiver) = channel.split();
            let t = thread::current();

            s.spawn(move || {
                // Some heavy calculation!
                thread::sleep(Duration::from_millis(100));
                sender.send("Hello, World!");
                t.unpark();
            });

            // receive() will block until data is ready.
            assert_eq!(receiver.receive(), "Hello, World!");
        });

        // OK to split here
        channel.split();
    }
}
