use std::io;
fn mul(x:i32, y:i32) -> i32 {
     x*y
}
fn div(x:i32, y:i32) -> i32 {
    x/y
}
fn rem(x:i32, y:i32) -> i32 {
    x%y
}
fn add(x:i32, y:i32) -> i32 {
    x+y
}
fn sub(x:i32, y:i32) -> i32{
    x-y
}

fn main() {
    println!("enter two numbers :");
    let mut a = String::new();
    io::stdin().read_line(&mut a)
        .expect("fail");
    let a: i32 = match a.trim().parse() {
        Ok(a) => a, 
        Err(_) => {
            println!("error");
            return;
        }
    };

    let mut b = String::new();
    io::stdin().read_line(&mut b)
        .expect("fail");
    let b: i32 = match b.trim().parse() {
        Ok(b) => b, 
        Err(_) => {
            println!("error");
            return;
        }
    };
    let mut op = String::new();
    io::stdin().read_line(&mut op)
        .expect("fail");
    let op: i32 = match op.trim().parse() {
        Ok(op) => op, 
        Err(_) => {
            println!("error");
            return;
        }
    };
    println!("enter the operation to perform : ");
    let ans =   match op 
        {
            1 => add(a,b),
            2 => sub(a,b),
            3 => mul(a,b),
            4 => div(a,b),
            5 => rem(a,b),
            _ => panic!("wtf dude?"),
        };
    println!("{} is the answer",ans);  
}
