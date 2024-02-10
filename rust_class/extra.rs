// Rust features : Fastest after C, No garbage collection(fast runtime), Memory safety
//Hello World program
fn main(){
    println!("Hello World");
    var();
    bleh();
    //Calling sum() func
    let s = sum(1,2);
    assert_eq!(s, 3);
    println!("Success!");
    //matching();
    str();
    str_deep();
    BOX();
}
//Variables are assigned using let keyword
//Difference between print!() and println!() is println!() adds newline at the end
//Variable initialization
fn var(){
    let x: i32 = 5; //Initialised to 5
    let _y: i32; //Unitialised, if used will give error, so underscore is used
    assert_eq!(x, 5); //Checks for equality
    println!("Success");
}
//mut keyword used to make variable mutable
//Scope - range within the program where item/variable is valid
//Unused variable warning removed using underscore before variable name
//This warning also removed using '#[allow(unused_variables)]'

//Char - single character of size 4 bytes, written in ''
//Bool - Boolean value of true or false of size 1byte
//Unit - Empty tuple of size 0 bytes, used to return nothing in expressions or functions
//Statement - perform action but does not produce a value
//Expression - evaluate to a resultant value

fn bleh(){
    let x: u32 = 5u32;
    let y = {
        let x_squared = x*x;
        let x_cube = x_squared * x;
        //This expression is assigned to y as it has no semicolon
        x_cube + x_squared + x
    };
    let z = {
        //2 * x;  Here z is of unit type and has no value assigned to it coz of the semicolon
        2 * x  //Since no semi colon this value is assigned to z
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn sum(x: i32, y:i32) -> i32{  //Return type of function i32
    x + y //If semi colon is given then return type would be unit/() as no value assigned to func
}
//assert_eq checks two values if they are equal. If equal - normal execution else, macro panics
//Functions - block of reusable code that performs specific tasks, takes arg and produces results
//Diverging functions - never return to the caller. Defined using ! as type
//Functions have to allocate types for all their arguments
/*fn matching(){
    let b = false;
    let _v = match b{ //match keyword works like switch condition in C
        true => 1,
        false => {
            println!("Success");
            panic!("we have no value for 'false', but we can panic");
        }
    };
    println!("Exercise failed if printing out this line");
}*/ //panic condition is printed here

//Ownership - set of rules that govrern memory management that are enforced in compile time
//Each value - owner, there can be only one owner, owner - out of scope - value dropped
//String - mutable - value can be changed at runtime
fn str(){   //String initialization
    let s1 = String::from("Hello");
    println!("{:?}", s1);
    let s2 = s1; //Now s1 cannot be used and s2 becomes owner of the string
}

//To do a deep copy use clone()
fn str_deep(){
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2={}", s1, s2);  //Here both s1 and s2 can be used
}

//Ownership prevents - Dangling pointers, Double-free, memory leaks
//&str - String slice - they have fixed size as they reference only portion of original string
//String - heap allocated string - mutable and can be dynamically resized
//Box - datatype that is a pointer which represents heap-allocated memory location

fn BOX(){
    let x: Box<i32> = Box::new(5);
    let mut y: Box<i32> = Box::new(1);
    *y = 4;
    assert_eq!(*x, 5); //x points to memory address, *x points to value in that address
    println!("Success");
}

//Partial move ...contd
