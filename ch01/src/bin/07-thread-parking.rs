use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

fn main() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                // Wait until being notified
                thread::park();
            }
        });

        // Producing thread
        for i in 0_u64 .. {
            queue.lock().unwrap().push_back(i);
            // Notify the consuming thread of the new data
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    })
}
