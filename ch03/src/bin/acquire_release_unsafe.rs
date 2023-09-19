use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        unsafe {
            DATA = 123;
        }
        READY.store(true, Ordering::Release); // Everything before this store ...
    });

    while !READY.load(Ordering::Acquire) {
        // ... is visible after this loads `true`
        thread::sleep(Duration::from_millis(100));
        println!("Waiting...");
    }

    unsafe {
        println!("{}", DATA);
    }
}
