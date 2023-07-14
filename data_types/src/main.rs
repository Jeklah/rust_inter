fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // floored = 0

    // Remainder
    let remainder = 43 % 5;

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The valuye of y is : {y}");

    // Accessing value by index of tuple.
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // Array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [3; 5];

    let b = [1, 2, 3, 4, 5];
    let first = b[0];
    let second = b[1];
}
