fn main() {
    let mut balance: i32 = 5000;
    println!("initial balance: {:?}", balance);
    deposit(&mut balance, 34);
    println!("after deposit: {:?}", balance);
    withdrawal(&mut balance, 500);
    println!("after withdrawal: {:?}", balance);
}

fn deposit(balance: &mut i32, amount: i32) {
    *balance += amount;
}

fn withdrawal(balance: &mut i32, amount: i32) {
    *balance -= amount;
}
