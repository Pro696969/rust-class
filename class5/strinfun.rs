fn main() {
    let name: String = String::from("Rust programming");
    display(name);
}

fn display(param_name:String) {
    println!("param_name value is : {}",param_name);
}
