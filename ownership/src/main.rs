fn main() {
    let a_num = 4;
    make_and_drop();
    let first = String::from("Ferris");
    // let full = add_suffix(first);
    // println!("{full}");

    let first_clone = first.clone();
    let full = add_suffix(first_clone);

    println!("{full}, originally: {first}");

    // Which of these compile compile?
    // A)
    let b = Box::new(0);
    let b2 = b;
    move_a_box(b);
    // B)
    let b = Box::new(0);
    move_a_box(b);
    println!("{b}");
    // C)
    let b = Box::new(0);
    move_a_box(b);
    let b2 = b;
    // D)  Only D will compile as the ownership of b is moved to b2 or when it is passed to move_a_box
    let b = Box::new(0);
    let b2 = b;
    move_a_box(b2);
}

fn make_and_drop() {
    let a_box = Box::new(5);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn move_a_box(b: Box<i32>) {}
