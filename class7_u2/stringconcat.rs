fn concat_strings_owned(string1: String, string2: &str) -> String {
    let mut concatenated_string = string1;
    concatenated_string.push_str(string2);
    concatenated_string
}

fn main() {
    let str1 = String::from("Hello, ");
    let str2 = "world!";
    
    let result = concat_strings_owned(str1, str2);
    println!("{}", result);

    // Since str1 has been moved into the function, it can't be used anymore in this scope
    // println!("{}", str1); // This will result in an error
}

