fn main() {
    let student_code="kk";
    let student = match student_code {
        "kk" => {println!("Student Found"); "kk"},
        "kx" => {println!("Student Found"); "kx"},
        "nk" => {println!("Student Found"); "nk"},
        "rd" => {println!("Student Found"); "rd"},
        _ => "unknown"
    };
    println!("student name is {}", student);
}
