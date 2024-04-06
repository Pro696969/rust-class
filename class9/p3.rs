#[derive(Debug)]
enum GenderCategory {
    Male, Female
}
struct person {
    name:&
}

fn main() {
    // let male = GenderCategory::Male;
    // let female = GenderCategory::Female;
    // println!("{:?}",male);
    // println!("{:?}",female);

    let p1 = person {
        name:String::from("Kx");
        gender:GenderCategory::Male
    };
    let p2 = person {
        name:String::from("KK");
        gender:GenderCategory::Female
    };
    
    println!("{:?}",p1);
    println!("{:?}",p2);


}
