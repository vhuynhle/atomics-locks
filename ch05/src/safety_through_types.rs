use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

/// Create a pair of (sender, receiver) which can be used
/// to send and receive one single message
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Arc::new(Channel::<T>::new());
    let channel_clone = channel.clone();

    (
        Sender { channel },
        Receiver {
            channel: channel_clone,
        },
    )
}

pub struct Sender<T> {
    // Using Arc because the channel is shared between the sender and the receiver
    channel: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    // Using Arc because the channel is shared between the sender and the receiver
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        // Now `send` takes `self` by value (and consumes it), ensuring that
        // `send` can be called only once.
        // No need to `panic` any more!
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Ordering::Release);
    }
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        // Use relaxed ordering, because the load inside `receive` with
        // acquire ordering will ensure the necessary synchronization with Sender::send.
        self.channel.ready.load(Ordering::Relaxed)
    }

    pub fn receive(self) -> T {
        // Now receive takes `self` by value and consumes it, ensuring that `receive` cannot be called
        // at most once.
        if !self.channel.ready.load(Ordering::Acquire) {
            panic!("No message available!");
        }

        // Safety: The check above ensure that message has been initialized
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

// Private now
struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    const fn new() -> Self {
        Channel {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
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
    use crate::safety_through_types::channel;
    use std::thread;

    #[test]
    fn can_send_and_receive_message_once() {
        thread::scope(|s| {
            let (sender, receiver) = channel();
            let t = thread::current();

            s.spawn(move || {
                sender.send("Hello, World!");
                // sender has been moved by this call, and cannot be used again.
                // Now sending twice is prohibited by the compiler!
                // sender.send("This will not compile!");

                // Wake-up the main thread
                t.unpark();
            });

            while !receiver.is_ready() {
                // Data is not ready, park the current thread
                thread::park();
            }

            assert_eq!(receiver.receive(), "Hello, World!");
        });
    }
}
