fn main(){
    println!("specifying data type");
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}", arr);
    println!("array size is {}", arr.len());
    println!("without specifying data type");
    let arr = [10,20,30,40];
    println!("array is {:?}", arr);
    println!("array size is {}", arr.len());
}
