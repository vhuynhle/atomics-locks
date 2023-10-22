#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Unimplemented for non-Linux OSes");
}

use std::{
    sync::atomic::{AtomicU32, Ordering},
    thread,
    time::Duration,
};

#[cfg(target_os = "linux")]
pub fn wait(a: &AtomicU32, expected: u32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,                    // The futex syscall
            a as *const AtomicU32,              // The atomic to operate on
            libc::FUTEX_WAIT,                   // The operation
            expected,                           // The expected value
            std::ptr::null::<libc::timespec>(), // no timeout
        );
    }
}

#[cfg(target_os = "linux")]
pub fn wake_one(a: &AtomicU32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,       // syscall
            a as *const AtomicU32, // atomic to operate on
            libc::FUTEX_WAKE,      // operation
            1,                     // number of thread to wake
        );
    }
}

fn main() {
    let a = AtomicU32::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(Duration::from_secs(3)); // Sleep for some time
            a.store(1, Ordering::Relaxed); // (1) Set the atomic variable
            wake_one(&a); // (2) wake up the main thread
        });

        println!("Waiting ...");
        while a.load(Ordering::Relaxed) == 0 {
            // (3) Wait as long as the variable is 0

            // (4) Put the thread to sleep. IMPORTANT: this operation checks if the value is
            // still zero before going to sleep so that the signal from the spawned thread (2)
            // does not get lost between (3) and (4).
            wait(&a, 0);
        }
        println!("Done");
    });
}
