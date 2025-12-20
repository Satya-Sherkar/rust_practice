// Basic addition function
use std::io;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // Read first number
    println!("Enter first number:");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let num1: i32 = input1.trim().parse().expect("Please enter a valid integer");

    // Read second number
    println!("Enter second number:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let num2: i32 = input2.trim().parse().expect("Please enter a valid integer");

    // Call add function
    let sum = add(num1, num2);
    println!("Sum is: {}", sum);
}
