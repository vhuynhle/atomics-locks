use std::{
    sync::atomic::{fence, AtomicBool, Ordering},
    thread,
    time::Duration,
};

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn some_calculation(i: u64) -> u64 {
    (i + 1) as u64
}

fn main() {
    // A thread waiting for data
    thread::spawn(|| {
        // Adjust this number to see different number of available results
        thread::sleep(Duration::from_millis(500));

        let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));
        if ready.contains(&true) {
            fence(Ordering::Acquire);

            for i in 0..10_usize {
                if ready[i] {
                    // ready[i] = true is observed by READY[i].load(Ordering::Relaxed)
                    // This means READY[i].store(true, Ordering::Release) happens-before
                    // READY[i].load(Ordering::Relaxed)
                    // This ensures that the assignment `DATA[i as usize] = data` happens before
                    // this line, and we can safely read the content of DATA[i]
                    // If ready[i] = false, reading the data may not be safe.
                    println!("data[{i}] = {}", unsafe { DATA[i] });
                }
            }
        }
    });

    // 10 thread producing data
    for i in 0..10_usize {
        thread::spawn(move || {
            let data = some_calculation(i as u64);
            unsafe {
                DATA[i as usize] = data;
            }

            // Equivalent to:
            // fence(Ordering::Release)
            // READY[i].store(true, Ordering::Relaxed)
            READY[i].store(true, Ordering::Release);
        });
        thread::sleep(Duration::from_millis(100));
    }
}
