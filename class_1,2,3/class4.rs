fn main(){
    ifelse();
    stringmatch();
    loops();
    scoop();
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

fn loops(){
    for x in 1..11{  //use 1..=11 to include 11
        if x==5{
            continue;
        }
        println!("x is {}", x);
    }
}

fn scoop(){
    let mut x = 0;
    while x<10{
        x+=1;
        println!("inside the loop x value is {}", x);
    }
    println!("outside the loop x value is {}", x);
}

fn scope(){
    let mut x = 0;

}
