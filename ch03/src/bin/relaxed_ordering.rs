use std::{
    collections::HashMap,
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

fn main() {
    static X: AtomicI32 = AtomicI32::new(0);
    static Y: AtomicI32 = AtomicI32::new(0);

    let observations = &mut HashMap::new();

    const EXPERIMENTS: i32 = 1_000_000;
    for exp in 0..EXPERIMENTS {
        if exp % 1000 == 0 {
            println!(
                "{}/{} ({:.1}%)",
                exp,
                EXPERIMENTS,
                (exp as f32 * 100_f32) / EXPERIMENTS as f32
            );
        }
        thread::scope(|s| {
            X.store(0, Ordering::SeqCst);
            Y.store(0, Ordering::SeqCst);

            s.spawn(|| {
                X.store(10, Ordering::Relaxed);
                Y.store(20, Ordering::Relaxed);
            });

            s.spawn(|| {
                let y = Y.load(Ordering::Relaxed);
                let x: i32 = X.load(Ordering::Relaxed);

                observations
                    .entry((x, y))
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
            });
        });
    }

    for ((x, y), count) in observations {
        println!("({}, {}) -> {}", x, y, count);
    }
}
