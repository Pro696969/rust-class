fn main(){
    let word = "HelloWorld";
    let bleh = match word{
        "HelloWorld" => {println!("HelloWorld found"); "HelloWorld"},
        "bye" => "gone",
        _ => "unknown"
    } ;
    println!("{}", bleh);
}
