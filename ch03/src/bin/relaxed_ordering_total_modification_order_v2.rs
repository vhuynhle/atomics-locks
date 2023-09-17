use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.fetch_add(5, Ordering::Relaxed);
}

fn a2() {
    X.fetch_add(10, Ordering::Relaxed);
}

fn b(observations: Arc<Mutex<HashMap<(i32, i32, i32, i32), usize>>>) {
    let a = X.load(Ordering::Relaxed);
    let b = X.load(Ordering::Relaxed);
    let c = X.load(Ordering::Relaxed);
    let d = X.load(Ordering::Relaxed);

    observations
        .lock()
        .unwrap()
        .entry((a, b, c, d))
        .and_modify(|count| {
            *count += 1;
        })
        .or_insert(1);
}

fn main() {
    let observations: Arc<Mutex<HashMap<(i32, i32, i32, i32), usize>>> =
        Arc::new(Mutex::new(HashMap::new()));
    const EXPERIMENTS: usize = 1_000_000;

    for i in 0..EXPERIMENTS {
        if i % 1000 == 0 {
            println!(
                "Progress: {:7} / {} ({:4.1}%)",
                i,
                EXPERIMENTS,
                (i * 100) as f32 / EXPERIMENTS as f32
            );
        }

        let observations = observations.clone();
        X.store(0, Ordering::Relaxed);
        thread::scope(|s| {
            s.spawn(a);
            s.spawn(a2);
            s.spawn(move || b(observations));
        });
    }

    observations
        .lock()
        .unwrap()
        .iter()
        .for_each(|((a, b, c, d), count)| {
            println!("{a} {b} {c} {d} --> {count}");
        });
}
