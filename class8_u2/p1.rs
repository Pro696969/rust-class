fn main() {
    let tup:(i32,&str,bool) = (1,"abcd",is_admin=true);
    println!("{} {} {}",tup.0,tup.1,tup.2);
}
