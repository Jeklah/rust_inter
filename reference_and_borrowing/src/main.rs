fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2);
    let s = format!("{m1}, {m2}!");

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             // so x points to 2

    let r1: &Box<i32> = &x; // &x borrows the Box, so r1 points to x on the stack
    let b: i32 = **r1; // two dereferences to get to read the heap value, so b = 2
    let r2: &i32 = &*x; // r2 points to the heap value directly, so r2 = 2
    let c: i32 = *r2; // so only one dereference to read the heap value, so c = 2
}

fn greet(g1: &String, g2: &String) {
    println!("{g1}, {g2}!");
}
