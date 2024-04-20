mod printfn;
mod calci;

fn main() {
    printfn::print("Hi kk");
    let sum = calci::add(5, 3);
    println!("5 + 3 = {}", sum);
    let diff = calci::subtract(10, 4);
    println!("10 - 4 = {}", diff);
    let mul = calci::multiply(10, 4);
    println!("10 * 4 = {}", mul);
    let div = calci::divide(10, 4);
    println!("10 // 4 = {}", div);

}
