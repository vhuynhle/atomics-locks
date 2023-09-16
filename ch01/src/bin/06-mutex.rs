use std::{sync::Mutex, thread, time::Duration};

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                println!("Thread {:?} trying to get lock", thread::current().id());
                let mut guard = n.lock().unwrap();
                println!("Thread {:?} acquired lock", thread::current().id());
                for _ in 0..100 {
                    *guard += 1;
                }
                thread::sleep(Duration::from_secs(1));
                println!("Thread {:?} releasing lock", thread::current().id());
            });
        }
    });

    assert_eq!(n.into_inner().unwrap(), 1000);
}
