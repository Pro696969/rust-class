use std::io;
fn mul(x:i32, y:i32) -> i32 {
     x*y
}
fn main() {
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
    
    println!("{}", mul(a,b));

}

