fn main() {
    let string1 = String::from("Hello, ");
    let string2 = "Nikhil!";
    let result = concatenate(string1, string2);
    println!("{}", result);
}

fn concatenate(s1: String, s2: &str) -> String {
    let concatenated_string = s1 + s2;
    concatenated_string
}
