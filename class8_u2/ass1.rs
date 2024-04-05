//complete the following programs and submit.
// 1. Implement an address book application, where each entry represents a person's contact information(name, email and phone no), use structure to represent each entry.
struct AddBook {
        name: String,
        phone: i64,
        email: String,
    }
fn main() {
    let person1 = AddBook {
        name:"KK".to_string(),
        phone: 9986558635,
        email:"kk@gmail.com".to_string()
    };

    let person2 = AddBook {
        name:"Ruspa".to_string(),
        phone: 9999999999,
        email:"Ruspa@mail.com".to_string()
    };

    println!("name of person is {} \nmail id is {} \nphone number is {}",person1.name,person1.email,person1.phone);
    println!("\nname of person is {} \nmail id is {} \nphone number is {}",person2.name,person2.email,person2.phone);

}
