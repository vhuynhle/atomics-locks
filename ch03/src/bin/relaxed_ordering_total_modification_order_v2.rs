use std::{
    collections::HashMap,
    fmt::Display,
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

#[derive(Eq, Hash, PartialEq)]
struct Observation {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Display for Observation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.a, self.b, self.c, self.d)
    }
}

fn b(observations: Arc<Mutex<HashMap<Observation, usize>>>) {
    let a = X.load(Ordering::Relaxed);
    let b = X.load(Ordering::Relaxed);
    let c = X.load(Ordering::Relaxed);
    let d = X.load(Ordering::Relaxed);
    let observation = Observation { a, b, c, d };

    observations
        .lock()
        .unwrap()
        .entry(observation)
        .and_modify(|count| {
            *count += 1;
        })
        .or_insert(1);
}

fn main() {
    let observations: Arc<Mutex<HashMap<Observation, usize>>> =
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
        .for_each(|(observation, count)| {
            println!("{observation} --> {count}");
        });
}
