// 2. create a simple employee management system where each employee has a name, age and department. use a structure to represent each employee.
struct Employee {
        name: String,
        age: i64,
        dept: String,
    }
fn main() {
    let person1 = Employee {
        name:"KK".to_string(),
        age: 16,
        dept:"CIVIL".to_string(),
    };

    let person2 = Employee {
        name:"GK(real)".to_string(),
        age: 17,
        dept:"MECH".to_string()
    };

    println!("name of person is {} \nage id is {} \ndept is {}",person1.name,person1.age,person1.dept);
    println!("\nname of person is {} \nage id is {} \ndept is {}",person2.name,person2.age,person2.dept);

}
