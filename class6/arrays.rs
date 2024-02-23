fn main() {
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is : {}",arr.len());
    for i in 0..4 {
        println!("index is {} and value is {}",i,arr[i]);
    }
}
