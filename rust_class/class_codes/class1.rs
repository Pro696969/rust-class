fn main(){
    println!("Hello World");
    print!("hello");
    print!("world");
    main1();
}
//fn - to define a function
//let - used for declaring a variable
//mut - mutable
//match - enables pattern matching
//while,for - loops and interation
//loop - creates infinite loop
//mod - defines the module
//use - brings an item into the scope
//struct - defines a structure
//pub - specifies visibility of an item
//static,constant - declares static and constant variable
//unsafe - allows unsafe operation
//as - typecasting
//break,continue
//return
//self,super - reference to current operation module
//crate - which package it belongs to
fn main1(){
    let company_string = "TutorialIsPoint"; //string
    let rating_float = 4.5; //float type
    let is_growing_boolean = true; //boolean type
    println!("company name is:{}", company_string);
    println!("company rating on 5 is:{}", rating_float);
    println!("company is growing:{}", is_growing_boolean);
}
