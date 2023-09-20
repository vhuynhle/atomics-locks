use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    const DELAY: bool = false;
    let a = thread::spawn(|| {
        // Warn the other thread that this thread is about to access S
        A.store(true, Ordering::SeqCst);

        if DELAY {
            thread::sleep(Duration::from_millis(20));
        }

        // Check that the other thread hasn't access S before changing S
        if !B.load(Ordering::SeqCst) {
            println!("Thread a: B has not been set to true, -> accessing S");
            thread::sleep(Duration::from_millis(20));
            unsafe {
                S.push('!');
            }
        }
    });

    let b = thread::spawn(|| {
        // Warn the other thread that this thread is about to access S
        B.store(true, Ordering::SeqCst);

        if DELAY {
            thread::sleep(Duration::from_millis(20));
        }

        // Check that the other thread hasn't access S before changing S
        if !A.load(Ordering::SeqCst) {
            println!("Thread b: A has not been set to true, -> accessing S");
            unsafe { S.push('!') };
        }
    });
    a.join().unwrap();
    b.join().unwrap();

    // How does it work?
    // The SeqCst ordering ensure a global single order.
    // In any scenario, the first operation is a store operation, which prevents the other
    // thread from accessing S.
    // Thus there is at most 1 thread can access S.
    // (There are scenarios in which the first 2 operations are stores, in which no thread accesses S.)

    unsafe {
        if S.is_empty() {
            println!("No thread accessed S, Ok");
        } else if S.len() == 1 {
            println!("One thread access S, Ok");
        } else {
            eprintln!("More than 1 thread accessed S -> FAIL!");
            std::process::exit(1);
        }
    }
}
