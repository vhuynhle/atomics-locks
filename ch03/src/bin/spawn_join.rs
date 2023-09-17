use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);

fn f() {
    let x = X.load(Ordering::Relaxed);
    assert!(x == 1 || x == 2);
}

fn main() {
    const EXPERIMENTS: i32 = 1_000_000;
    for i in 0..EXPERIMENTS {
        if i % 1000 == 0 {
            println!(
                "Progress: {:7} / {} ({:4.1}%)",
                i,
                EXPERIMENTS,
                (i * 100) as f32 / EXPERIMENTS as f32
            );
        }

        X.store(1, Ordering::Relaxed);
        let t = thread::spawn(f);
        X.store(2, Ordering::Relaxed);
        t.join().unwrap();
        X.store(3, Ordering::Relaxed);

        let x = X.load(Ordering::Relaxed);
        assert_eq!(x, 3);
    }
    println!("All assertions satisfied");
}
