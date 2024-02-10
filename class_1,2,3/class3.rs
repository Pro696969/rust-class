fn main(){
    let age:u8=255;
    let weight:u8=254;
    let height:u8=253;
    let score:u8=252;
    let float_with_separator = 11_000.5;
    let int_with_separator = 50_000;
    println!("float value : {}", float_with_separator);
    println!("int value : {}", int_with_separator);
    println!("age is : {}", age);
    println!("weigth is : {}", weight);
    println!("height is : {}", height);
    println!("score is : {}", score);
    let x = 42;
    println!("original x : {}", x);
    let x = "shadowed";
    println!("shadowed x : {}", x);
    let x = true;
    println!("boolean x : {}", x);
    let y:i32 = 10;
    println!("original y : {}", y);
    let y = y*2;
    println!("doubled y : {}", y);
}
