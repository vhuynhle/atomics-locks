use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        // Safety: We're holding the exclusive lock, so nothing else is accessing DATA.
        unsafe {
            DATA.push('!');
        }
        LOCKED.store(false, Ordering::Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });

    unsafe {
        println!("{}, length: {}", DATA, DATA.len());
    }
}
