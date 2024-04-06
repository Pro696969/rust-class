fn main() {
    let result = is_even(2);
    println!("{:?}",result);
    let result = is_even(3);
    println!("{:?}",result);
}

fn is_even(no:i32)->Option<bool> {
    if no%2 == 0 {
        Some(true)

    }
    else {
        None
    }
}
