use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

/// Usage example of a stop flag with relaxed memory ordering.
/// This ordering is OK because we're working on one (1) flag.
fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Working in the background
    let background_thread = thread::spawn(|| {
        while !STOP.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Listen to user input in the main thread
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd}"),
        }
    }

    // Request the background thread to stop
    STOP.store(true, Ordering::Relaxed);

    background_thread.join().unwrap();
}
