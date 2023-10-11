use std::{
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Instant,
};

static A: AtomicU64 = AtomicU64::new(0);

fn main() {
    black_box(&A);
    thread::spawn(|| loop {
        A.store(0, Ordering::Relaxed);
    });

    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Ordering::Relaxed));
    }
    println!("{:?}", start.elapsed());
}
