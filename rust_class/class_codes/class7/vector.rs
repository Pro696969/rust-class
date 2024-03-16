fn main(){
    let v = vec![1,2,3];
    let _v2 = v.clone();   //without clone - error, underscore used to remove warning of unused variable
    println!("{:?}", v);
}
