use std::thread;

fn main() {
    let numbers = [1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("Length: {}", numbers.len());
        });

        s.spawn(|| {
            for n in numbers {
                println!("{}", n);
            }
        });
    });
}
