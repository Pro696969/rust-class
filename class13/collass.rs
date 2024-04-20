use std::collections::HashMap;

fn main() {
    let mut list = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", list);

    list.push(6);
    println!("After push: {:?}", list);

    list.pop(); 
    println!("After pop: {:?}", list);

    list.sort(); 
    println!("Sorted vector: {:?}", list);

    
    let mut colors = HashMap::new();
    colors.insert("red".to_string(), 3);
    colors.insert("yellow".to_string(), 2);
    colors.insert("orange".to_string(), 5);
    println!("Colors: {:?}", colors);

    let count = colors.get(&"yellow".to_string()).unwrap_or(&0); 
    println!("Yellow count: {}", count);

    colors.remove(&"red".to_string()); 
    println!("colors after removal: {:?}", colors);

    
    let mut a = String::from("Hi im kk, ");
    println!("Greeting: {}", a);

    a.push_str("hi im kx"); 
    println!("Greeting after append: {}", a);

    let uppercase = a.to_uppercase(); 
    println!("Uppercase: {}", uppercase);

    let contains_word = a.contains("word"); 
    println!("Contains 'world'? {}", contains_word);
}
