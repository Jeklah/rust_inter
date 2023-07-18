fn main() {
    let a_num = 4;
    make_and_drop();
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
}

fn make_and_drop() {
    let a_box = Box::new(5);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
