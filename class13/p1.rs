use std::io::Read;
fn main() {
    let mut file = std::fs::File::open("data2.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}",content);
}
