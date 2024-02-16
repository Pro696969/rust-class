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
    println!("enter the operation to perform : ");
    println!("1 for addition");
    println!("2 for subtraction");
    println!("3 for multiplication");
    println!("4 for division");
    println!("5 for remainder");

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
    if op==1 {
        println!("{}",add(a,b));

    }
    if op==2 {

        println!("{}",sub(a,b));
    }
    if op==3 {

        println!("{}",mul(a,b));
    }
    if op==4 {

        println!("{}",div(a,b));
    }
    if op==5 {

        println!("{}",rem(a,b));
    }
    
}
