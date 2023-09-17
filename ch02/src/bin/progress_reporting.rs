use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

/// Example or relaxed memory ordering.
/// This order is OK because the two threads operate on the same
/// variable.
fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread doing the work
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Ordering::Relaxed);
            }
        });

        // The main thread shows status update
        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working... {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    })
}

fn process_item(_: usize) {
    thread::sleep(Duration::from_millis(100));
}
