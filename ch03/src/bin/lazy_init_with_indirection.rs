use std::{
    sync::atomic::{AtomicPtr, Ordering},
    thread,
    time::Duration,
};

struct Data {
    v: i32,
}

fn generate_data() -> Data {
    thread::sleep(Duration::from_millis(50));

    Data { v: 42 }
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Ordering::Acquire);

    if p.is_null() {
        println!(
            "Thread {:?} loads NULL, trying to initialize data",
            thread::current().id()
        );

        // Generate new data with generate_data
        // Store it in a new allocation with Box::new
        // Turn this box into a pointer with Box::into_raw,
        // so that we can attempt to store it in PTR
        p = Box::into_raw(Box::new(generate_data()));

        // Attempt to store p in PTR
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(), // Check against std::ptr::null_mut()
            p,                    // If the same, then will set PTR to p
            Ordering::Release, // The ordering for the read-modify-write operation if the comparison above succeeds
            Ordering::Acquire, // The ordering for the load operation if the comparison above fails
        ) {
            println!(
                "Thread {:?} loses the race to initialize data.",
                thread::current().id()
            );

            // PTR was already set to e: Another thread won the race to initialize data.

            // The ordering is Acquire. Together with the Release ordering of the winning thread,
            // we can ensure that the initialization of the pointer e happens-before the
            // assignment of e to p below.
            // In other word, these orderings ensure that p points to initialized data.

            // Here we turn the raw pointer back into a box and drop it.
            drop(unsafe { Box::from_raw(p) });

            // Use the pointer from the thread that won the race.
            p = e;
        } else {
            println!(
                "==================== Thread {:?} did the initialization! ====================",
                thread::current().id()
            );

            // PTR has been atomically set to p
            // The ordering is Release
        }
    } else {
        // PTR already pointed to valid data
        println!("Thread {:?}: get existing data", thread::current().id());
    }

    unsafe { &*p }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                let v = get_data().v;
                println!("Thread {:?}: data = {}", thread::current().id(), v);
            });

            thread::sleep(Duration::from_millis(5));
        }
    });
}
