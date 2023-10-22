use std::{
    sync::atomic::{compiler_fence, AtomicBool, AtomicUsize, Ordering},
    thread,
};

// In this experiment, we use the wrong ordering for the atomic operations
// (Relaxed instead of Acquire/Release).
// However, on x86_64, it produces the expected result (4_000_000)
// because x86_64 is a strongly ordered architecture.
fn main() {
    let locked = AtomicBool::new(false);
    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..1_000_000 {
                    // Acquire the lock, using the wrong memory ordering
                    while locked.swap(true, Ordering::Relaxed) {}

                    // Ensure that the compiler doesn't reorder our operations
                    compiler_fence(Ordering::Acquire);

                    // Non-atomically increasing the counter while holding the lock
                    let old = counter.load(Ordering::Relaxed);
                    let new = old + 1;
                    counter.store(new, Ordering::Relaxed);

                    // Release the lock, using the wrong memory ordering
                    compiler_fence(Ordering::Release);
                    locked.store(false, Ordering::Relaxed);
                }
            });
        }
    });

    println!("{}", counter.into_inner());
}
