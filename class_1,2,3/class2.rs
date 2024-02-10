fn main(){
    println!("Hello World");
    let age:i32 = 20;
    let is_value:bool = true;
    let name:&str = "KDB";
    println!("Name:{}, Age:{}, Is_value:{}", name, age, is_value);
    main1();
    main2();
    main3();
}

use std::io;
fn main1(){
    println!("Enter value");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("you entered:{}", input.trim());
}

fn main2(){
    //integer types
    let integer_number:i32=42;
    let unsigned_integer:u64=123;
    //floating point type
    let float_number:f64=3.14;
    //boolean type
    let is_true:bool=true;
    //character type
    let character:char='A';
    //string type
    let string_text:String=String::from("Hello Rust!");
    //tuple type
    let tuple_example:(i32,f64,char)=(10, 3.5, 'A');
    //array type
    let array_example:[i32;3]=[1,2,3];
    println!("Integer:{}", integer_number);
    println!("Unsigned integer:{}", unsigned_integer);
    println!("Float:{}", float_number);
    println!("Boolean:{}", is_true);
    println!("String:{}", string_text);
    println!("Tuple:{:?}", tuple_example);
    println!("Array:{:?}", array_example);
}

fn main3(){
    println!("Enter value");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    //convert the input to a number
    let number:i32=match input.trim().parse(){
        Ok(parsed)=>parsed,
        Err(_)=>{
            println!("invalid input");
            return;
        }
    };
    println!("you entered:{}", number);
}
