
### SRN: PES2UG22CS281
### Name: M C Krishna Kumar
### E section


# 1. basic calculator

```rust
// Simple calculator
use std::io;
fn main() {
    println!("Enter your input1:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
    .expect("Failed to readline");
    let num1: i32 = match input1.trim().parse() {
        Ok(num1) => num1,
        Err(_) => {
            println!("Error");
            return;
        }
    };

    println!("Enter your input2:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
    .expect("Failed to readline");
    let num2: i32 = match input2.trim().parse() {
        Ok(num2) => num2,
        Err(_) => {
            println!("Error");
            return;
        }
    };

    println!("Addition result = {}",num1 + num2);
    println!("Subtraction result = {}",num1 - num2);
    println!("Multiplication result = {}",num1 * num2);
    println!("Division result = {}",num1 / num2);
    println!("Reminder result = {}",num1 % num2);

}
```


# Output 

![[Pasted image 20240202161700.png]]



