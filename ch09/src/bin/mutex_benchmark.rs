use std::time::Instant;

use ch09::{mutex_v1, mutex_v2};

fn main() {
    let m = mutex_v1::Mutex::new(0);

    // Force the compiler to assume there may be more code accessing the mutex
    std::hint::black_box(&m);

    let start = Instant::now();

    for _ in 0..5_000_000 {
        *m.lock() += 1;
    }
    
    let duration = start.elapsed();
    println!("v1: locked {} times in {:?}", *m.lock(), duration);

    let m = mutex_v2::Mutex::new(0);
    std::hint::black_box(&m);
    let start = Instant::now();
    for _ in 0..5_000_000 {
        *m.lock() += 1;
    }
    let duration = start.elapsed();
    println!("v2: locked {} times in {:?}", *m.lock(), duration);
    
}
