use std::{sync::Arc, thread, time::Duration};

fn main() {
    let c = Arc::new(ch05::unsafe_oneshot_channel::Channel::<i32>::new());

    let c1 = c.clone();
    let t1 = thread::spawn(move || {
        while !c1.is_ready() {
            std::hint::spin_loop();
        }
        let value = unsafe { c1.receive() };

        println!("Received value: {}", value);
    });

    let t2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        unsafe {
            c.send(1000);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
