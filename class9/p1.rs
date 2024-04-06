fn main() {
    let s1:String=String::from("Hello World");
    let s2=[1,2,3,4,5];
    let s3: &str=&s1[1..=4];
    println!("{:?}",s3);
    println!("{:?}",&s2[1..=2]);
}
