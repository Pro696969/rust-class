//1. Create a program that calculates the sum of elements in a slice of integers
fn main() {
    let v = vec![1,2,3,4,5];
    let a = &v[1..3];
    println!("original vector : {:?}",v);
    println!("slices vector : {:?}",a);
    let mut s = 0;
    for i in a.iter() {
        s = s + *i;
    }
    // let s = sum(a);
    println!("sum of sliced = {}",s);
}

