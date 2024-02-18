use std::io;
fn add(x:i32, y:i32)->i32{
    x+y
}
fn sub(x:i32, y:i32)->i32{
    x-y
}
fn mul(x:i32, y:i32)->i32{
    x*y
}
fn div(x:i32, y:i32)->i32{
    x/y
}
fn Mod(x:i32, y:i32)->i32{
    x%y
}

fn main() {
    let mut a = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: i32 = a.trim().parse().expect("Please enter a valid number");

    let mut b = String::new();
    println!("Enter the second number:");
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: i32 = b.trim().parse().expect("Please enter a valid number");

    println!("Choose operation:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Modulus");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    let result = match choice {
        1 => add(a, b),
        2 => sub(a, b),
        3 => mul(a, b),
        4 => div(a, b),
        5 => Mod(a, b),
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    println!("Result: {}", result);
}
