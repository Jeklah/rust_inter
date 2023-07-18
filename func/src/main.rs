fn hello() {
    println!("Hello world!");

    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn f(x: i32) -> i32 {
    x + 1
}
fn main() {
    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );
}
