use std::{
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Instant,
};

#[repr(align(64))]
struct Aligned(AtomicU64);

static A: [Aligned; 3] = [
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
];

fn main() {
    black_box(&A);
    thread::spawn(|| loop {
        A[0].0.store(0, Ordering::Relaxed);
        A[2].0.store(0, Ordering::Relaxed);
    });

    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A[1].0.load(Ordering::Relaxed));
    }
    println!("{:?}", start.elapsed());
}
