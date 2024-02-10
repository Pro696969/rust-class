
fn main(){
    ifelse();
    stringmatch();
    forloop();
    whileloop();
    loops();
}

fn ifelse(){
    let num:i32 = 5;
    if num>0{
        println!("{} is positive", num);
    }
    else if num<0{
        println!("{} is negative", num);
    }
    else{
        println!("{} is zero", num);
    }
}

fn stringmatch(){
    let state_code = "KA";
    let state = match state_code {
        "KA" => {println!("Found match for KA"); "Karnataka"},
        "MH" => "Maharashtra",
        "KL" => "Kerala",
        "GA" => "Goa",
        _ => "unknown"
    };
    println!("state name is {}", state);
}

fn forloop(){
    for x in 1..11{  //use 1..=11 to include 11
        if x==5{
            continue;
        }
        println!("x is {}", x);
    }
}

fn whileloop(){
    let mut x = 0;
    while x<10{
        x+=1;
        println!("inside the loop x value is {}", x);
    }
    println!("outside the loop x value is {}", x);
}

fn loops(){
    let mut x = 0;
    loop{
        x += 1;
        println!("x = {}", x);
        if x == 10{
            break;
        }
    }
}

