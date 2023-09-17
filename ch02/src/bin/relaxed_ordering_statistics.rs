use std::{
    sync::atomic::{AtomicU64, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // 4 background thread doing work
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken, Ordering::Relaxed);
                }
            });
        }

        // Main thread prints status updates
        loop {
            // Because the 3 atomic variables are updated separately,
            // it is possible that
            // 1. The main thread loads total_time after a thread has updated num_done but before
            //    it has updated total_time. This results in an underestimate of the average.
            // 2. Because the Relaxed ordering does not guarantee the relative order in another
            //    thread, the main thread may briefly an updated value of total_time and an old value
            //    of num_done. This results in an overestimate of the average.
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);

            if n == 0 {
                println!("Working, nothing done yet.");
            } else {
                println!(
                    "Working {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
                if n == 100 {
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    })
}

fn process_item(_: u64) {
    thread::sleep(Duration::from_millis(200));
}
