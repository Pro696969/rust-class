use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    println!("Enter the second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Invalid input");
    let num2: f64 = num2.trim().parse().expect("Invalid input");
    let add = num2 + num1;
    let sub = num2 - num1;
    let mul = num2 * num1;
    let div = num2 / num1;
    let modulus = num2 % num1;
    println!("Addition: {}", add);
    println!("Subtraction: {}", sub);
    println!("Multiplication: {}", mul);
    println!("Division: {}", div);
    println!("Modulus: {}", modulus);
}

