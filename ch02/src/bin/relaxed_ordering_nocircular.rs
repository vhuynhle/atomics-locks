// An example from [[https://en.cppreference.com/w/CPO/atomic/memory_order][C++ Memory Order]],
// ported to Rust.

use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicI64, Ordering},
    thread,
};

fn main() {
    let x = &AtomicI64::new(0);
    let y = &AtomicI64::new(0);

    let f1 = move || {
        let r1 = y.load(Ordering::Relaxed);
        if r1 == 42 {
            x.store(r1, Ordering::Relaxed);
        }
    };

    let f2 = move || {
        let r2 = x.load(Ordering::Relaxed);
        if r2 == 42 {
            y.store(42, Ordering::Relaxed);
        }
    };

    let mut results: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    const TOTAL: usize = 1_000_000;
    for i in 0..TOTAL {
        if i % 1000 == 0 {
            println!(
                "{:7} / {} ({:.2}%)",
                i,
                TOTAL,
                (i * 100) as f32 / TOTAL as f32
            );
        }

        // Set x and y to 0 before starting the new threads
        x.store(0, Ordering::SeqCst);
        y.store(0, Ordering::SeqCst);

        // Start the threads and wait for them to finish
        thread::scope(|s| {
            s.spawn(f1);
            s.spawn(f2);
        });

        let x = x.load(Ordering::SeqCst);
        let y = y.load(Ordering::SeqCst);
        results
            .entry((x, y))
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }

    for ((x, y), count) in results.iter() {
        println!(
            "({}, {}) -> {} ({:.2}%)",
            x,
            y,
            count,
            (count * 100) as f32 / TOTAL as f32
        );
    }
}
