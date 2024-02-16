//pass by ref
fn main() {
    let mut no: i32 = 5;
    println!("The value of no is: {}",no);
    mutate_no_zero(& mut no);
    println!("The value of no is: {}",no);
}

fn mutate_no_zero(param_no:& mut i32) {
    *param_no = 0;
    println!("param_no value is: {}",param_no);
}
