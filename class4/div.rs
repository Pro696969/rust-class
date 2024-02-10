
//     line.parse().unwrap()
// }

fn main() {
    let state_code="KA";
    let state = match state_code {
        "KA" => {println!("match found for KA"); "Karnataka"},
        "MH" => "Maharashtra",
        "GA" => "Goa",
        _ => "unknown"
    };
    println!("state name is {}", state);
}
