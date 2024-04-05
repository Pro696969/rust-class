#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    is_admin: bool,
}

fn main() {
    // let mut user=User{
    //     id:1,
    //     name:"KK".to_string(),
    //     is_admin:true
    // };
    let mut user=get_user(String::from("Alice"));

    println!("{} {} {}",user.id,user.name,user.is_admin);
    user.is_admin = false;
    println!("{} {} {}",user.id,user.name,user.is_admin);
    //
    let name:String::from("BoB");
    ..user
}

fn get_user(name:String)->User {
    User {
        id:1,
        name:name,
        is_admin:true,
    }
}
