use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

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
    // The maximum ID that should be exceeded
    const LIMIT: u64 = 1000;

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    let mut id = NEXT_ID.load(Ordering::Relaxed);

    // Put in a loop to ensure the work is done,
    // when compare_exchange returns Ok(_).
    loop {
        // Check and panic before modifying NEXT_ID. This ensures that
        // NEXT_ID will never overflow the limit.
        assert!(id < LIMIT, "Too many IDs");

        match NEXT_ID.compare_exchange(id, id + 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => {
                // OK: NEXT_ID has not been modified by other threads
                // after we loaded it.
                return id;
            }
            Err(v) => {
                // NEXT_ID has been modified by some other thread(s)
                // after we loaded it. Try again with the updated value.
                id = v;
            }
        }
    }
}
