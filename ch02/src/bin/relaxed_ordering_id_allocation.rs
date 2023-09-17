use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

fn main() {
    let mut handles = vec![];
    for _ in 0..20 {
        handles.push(thread::spawn(|| {
            let id = allocate_new_id();
            println!("Thread {:?} -> {}", thread::current().id(), id);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn allocate_new_id() -> u64 {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
