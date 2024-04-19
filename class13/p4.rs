use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut fa = OpenOptions::new()
        .append(true)
        .open("data2.txt")
        .expect("cannot open file");

    fa
        .write("this is a new line i appended".as_bytes())
        .expect("write failed");

    println!("Appended content to a file");
}
