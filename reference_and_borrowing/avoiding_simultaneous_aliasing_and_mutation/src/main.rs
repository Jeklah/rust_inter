fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &vec[0];
    vec.push(4);
    println!("Third element is {}", *num);
}
