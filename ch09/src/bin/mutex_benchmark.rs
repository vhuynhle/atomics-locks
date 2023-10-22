use std::time::Instant;

use ch09::mutex_v1::Mutex;

fn main() {
    let m = Mutex::new(0);

    // Force the compiler to assume there may be more code accessing the mutex
    std::hint::black_box(&m);

    let start = Instant::now();

    for _ in 0..5_000_000 {
        *m.lock() += 1;
    }
    
    let duration = start.elapsed();
    println!("locked {} times in {:?}", *m.lock(), duration);
}
