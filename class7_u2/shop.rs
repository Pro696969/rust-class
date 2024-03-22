fn add_item(cart: &mut Vec<String>, item: String) {
    cart.push(item.clone());
    println!("Added {} to the cart.", item);
}

fn remove_item(cart: &mut Vec<String>, item: &str) {
    if let Some(index) = cart.iter().position(|x| x == item) {
        let removed_item = cart.remove(index);
        println!("Removed {} from the cart.", removed_item);
    } else {
        println!("Item {} not found in the cart.", item);
    }
}

fn display_cart(cart: &Vec<String>) {
    if cart.is_empty() {
        println!("Your cart is empty.");
    } else {
        println!("Items in your cart:");
        for item in cart {
            println!("{}", item);
        }
    }
}

fn main() {
    let mut cart: Vec<String> = Vec::new();

    add_item(&mut cart, String::from("ball"));
    add_item(&mut cart, String::from("pen"));
    add_item(&mut cart, String::from("krishikesh"));

    display_cart(&cart);

    remove_item(&mut cart, "pen");

    display_cart(&cart);
}
