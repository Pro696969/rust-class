fn main() {
    let v = vec![1,2,3];
    let v2=v;

    println!("In main {:?}",v2);
    display(v2.clone());
    println!("In main {:?}",v2);
}
fn display(v:Vec<i32>) {
    println!("Inside Display {:?}",v);
}
