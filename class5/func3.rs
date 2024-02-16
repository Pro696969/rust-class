fn main() {
    let no: i32 = 5;
    mutate_no_zero(no);
    println!("The value of no is: {}",no);
    mutate_no_zero(no);
    println!("The value of no is: {}",no);
}

fn mutate_no_zero(mut param_no:i32) {
    param_no = param_no * 0;
    println!("param_no value is: {}",param_no);
}
