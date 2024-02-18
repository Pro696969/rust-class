fn main(){
    call_by_ref();
    call_by_value();
}

fn call_by_ref(){
    let mut no: i32 = 5;
    println!("The value of no is: {}", no);
    println!("Call by reference");
    mutate_no_zero(& mut no);
    println!("The value of no is: {}", no);
}

fn mutate_no_zero(param_no: & mut i32){
    *param_no = 0;
    println!("param_no is: {}", param_no);
}

fn call_by_value(){
    let no: i32 = 6;
    println!("The value of no is: {}", no);
    println!("Call by value");
    mutate_no_of_zero(no);
    println!("The value of no is: {}", no);
}

fn mutate_no_of_zero(mut param_no: i32){
    param_no = param_no*0;
    println!("param_no value is: {}", param_no);
}

