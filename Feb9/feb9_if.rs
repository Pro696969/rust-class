use std::io;
fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}

fn main() {
    println!("input a number");
    let n1 = read_i32();
    println!("input another number");
    let n2 = read_i32();
    // let num:i32 = 5;
    if n1 > 0 {
        println!("num is positive");
    }
}
