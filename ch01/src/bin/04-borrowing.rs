fn main() {
    let a = 100;
    let mut b = 101;

    println!("Before: a = {a}, b = {b}");
    f(&a, &mut b);

    println!("After: a = {a}, b = {b}");
}

fn f(a: &i32, b: &mut i32) {
    println!("--- function start: a = {a}, b = {b}");
    *b += 1;
}
