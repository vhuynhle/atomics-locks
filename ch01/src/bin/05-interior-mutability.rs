use std::cell::{Cell, RefCell, RefMut};

fn main() {
    cell_demo();

    refcell_demo();
}

fn cell_demo() {
    let a = Cell::new(1);
    println!("Before: &a = {:p}", a.as_ptr());
    f(&a, &a);
    println!("After: &a = {:p}", a.as_ptr());

    let a = Cell::new(vec![1, 2, 3]);
    g(&a);
    let a = a.take();
    println!("After changing a vector inside a cell: {a:?}");
}

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("cell_demo: before != after");
    }
}

fn g(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take();
    v2.push(4);
    v.set(v2);
}

fn refcell_demo() {
    let v = RefCell::new(vec![1, 2, 3]);
    modify_refcell(&v);
    println!("Refcell: After chaning interior: {:?}", v.take());

    let rv = v.borrow_mut();
    refcell_panic(rv);
}

fn modify_refcell(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(4);
}

fn refcell_panic(mut v: RefMut<Vec<i32>>) {
    v.push(4);
}
