// take array input from user using function and print it 
// print array then modify the elements and print the array 
use std::io;
fn arry_input() {
    let mut numbers: [i32; 4] = [0; 4]; 
    for i in 0..4 {
        let mut input = String::new();
        println!("Enter a number:");

        io::stdin().read_line(&mut input).expect("Failed to read input");

        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        numbers[i] = number;
    }

    println!("The numbers you entered are: {:?}", numbers);

    println!("updated values are (all values made 10) : ");
    update(& mut numbers);
}

fn update(arr:&mut[i32;4]) {
    let mut sum = 0;
    for i in 0..4 {
        arr[i] = 10;
        sum=sum+arr[i];
    }
    println!("inside update {:?}",arr);
    println!("sum of all elements = {:?}",sum);
}

fn main() {
    arry_input();
    
}

