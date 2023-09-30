use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        // Push the message to the back of the queue
        // and notify one waiting receiver (if there are some)
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        // Lock the mutex
        let mut b = self.queue.lock().unwrap();
        loop {
            // Pop one message from the queue, if there are some
            if let Some(message) = b.pop_front() {
                return message;
            }

            // Otherwise, block until notified
            // Note that the `wait` function unlock the mutex while waiting (and re-lock
            // before returning), so this does not block the mutex while waiting.
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}
