use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from main thread: {:?}", thread::current().id());

    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = Vec::from_iter(1..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("Average: {}", average);
}

fn f() {
    println!("Hello from thread {:?}", thread::current().id());
}
