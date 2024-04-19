use std::io::Write;
fn main() {
    let mut file = std::fs::File::create("data2.txt").expect("create failed");
    file.write_all("Hello World".as_bytes()).expect("write failed");
    file.write_all("\nRust Programming is fUn".as_bytes()).expect("write failed");
    file.write_all("\nHi im kk".as_bytes()).expect("write failed");
}
