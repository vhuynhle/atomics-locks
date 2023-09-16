use std::rc::Rc;
use std::sync::Arc;
use std::thread;

static X: [i32; 3] = [1, 2, 3];

fn main() {
    share_statics();
    leak_box();
    rc_demo();
    arc_demo();
}

fn arc_demo() {
    println!("Reference counting with Arc (multi-threaded)");
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    let t1 = thread::spawn(move || dbg!(a));
    let t2 = thread::spawn(move || dbg!(b));
    t1.join().unwrap();
    t2.join().unwrap();
    println!();
}

fn rc_demo() {
    println!("Reference counting with Rc (single-threaded)");
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr());
}

fn leak_box() {
    println!("Leaking a box ...");
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    let t1 = thread::spawn(move || dbg!(x));
    let t2 = thread::spawn(move || dbg!(x));
    t1.join().unwrap();
    t2.join().unwrap();
    println!();
}

fn share_statics() {
    println!("Sharing statics");
    let t1 = thread::spawn(|| dbg!(X));
    let t2 = thread::spawn(|| dbg!(X));
    t1.join().unwrap();
    t2.join().unwrap();
    println!();
}
