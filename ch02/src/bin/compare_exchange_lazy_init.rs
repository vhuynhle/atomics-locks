// Lazy initialization using compare and exchange.
// This version allows initialization of non-constant values.

use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Duration,
};

use rand::{rngs::OsRng, RngCore};

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);

    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        println!("{:?} Generating the key...", thread::current().id());
        // generate_random_key must not return 0.
        // If generate_random_key can return 0, then it is possible that
        // a thread generates that value (0), then another thread overwrites KEY
        // with another non-zero value.
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => {
                println!(
                    "{:?} =======> Key updated by this thread <=======",
                    thread::current().id()
                );
                new_key
            }
            Err(k) => {
                println!("{:?} _-_ Someone was faster _-_", thread::current().id());
                k
            }
        }
    } else {
        println!("{:?} [key already generated]", thread::current().id());
        key
    }
}

fn generate_random_key() -> u64 {
    // Doing some heavy calculation here!
    thread::sleep(Duration::from_millis(200));

    loop {
        let key = OsRng.next_u64();
        if key != 0 {
            return key;
        }
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..20 {
            s.spawn(|| {
                println!("Key: {}", get_key());
            });
            thread::sleep(Duration::from_millis(20));
        }
    });
}
