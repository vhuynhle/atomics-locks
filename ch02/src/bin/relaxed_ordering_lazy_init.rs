use std::{sync::atomic::{AtomicU64, Ordering}, thread, time::Duration};

fn lazy_init_x() -> u64 {
    const SENTINEL: u64 = 0;
    static X: AtomicU64 = AtomicU64::new(SENTINEL);
    let mut x = X.load(Ordering::Relaxed);
    if x == SENTINEL {
        x = calculate_constant_x();
        X.store(x, Ordering::Relaxed);
    }
    x
}

fn calculate_constant_x() -> u64 {
    println!("Calculating x ...");
    thread::sleep(Duration::from_millis(30));
    42
}

fn main() {
    // There is /no data race/, but a race condition is still possible.
    // Two (or more) thread can see x == SENTINEL, and all of them will
    // do calculate_x() and try to write to X. The last thread doing so
    // will write the final result. In this example, this is ok because
    // x is a constant.
    thread::scope(|s| {
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
        s.spawn(lazy_init_x);
        thread::sleep(Duration::from_millis(8));
    });
}
