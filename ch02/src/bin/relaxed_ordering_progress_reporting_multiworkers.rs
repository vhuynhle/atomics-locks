use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

/// Example or relaxed memory ordering.
/// This order is OK because the two threads operate on the same
/// variable.
fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // 4 background threads doing the work
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }

        let start = Instant::now();
        // The main thread shows status update
        loop {
            let n = num_done.load(Ordering::Relaxed);
            println!(
                "{:.2?} Working... {n}/100 done",
                Instant::now().duration_since(start)
            );
            if n == 100 {
                break;
            }

            thread::sleep(Duration::from_secs(1));
        }
    })
}

fn process_item(_: usize) {
    thread::sleep(Duration::from_millis(400));
}
