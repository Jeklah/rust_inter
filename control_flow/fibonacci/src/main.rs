// this is a rust script for the fibonacci sequence for nth number.
// By: Jeklah
use std::io;

// function to find the nth fibonacci number
fn fib(n: i32) -> i32 {
    // base case
    if n <= 1 {
        return n;
    }
    // recursive case
    return fib(n - 1) + fib(n - 2);
}

// main function
fn main() {
    // enter a number from user
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // convert to int
    let input: i32 = input.trim().parse().expect("Please type a number!");

    // call the fib function
    let result = fib(input);
    println!("The {}th fibonacci number is {}", input, result);
}
