// 2. A program that simulates a traffic light using the enum and prints the current state of the traffic light.
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let mut current_state = TrafficLight::Red;
    let mut counter = 10;

    loop {
        println!("The current state of the traffic light is: {:?}", current_state);

        current_state = match current_state {
            TrafficLight::Red => {
                println!("Changing to green...");
                TrafficLight::Green
            }
            TrafficLight::Green => {
                println!("Changing to yellow...");
                TrafficLight::Yellow
            }
            TrafficLight::Yellow => {
                println!("Changing to red...");
                TrafficLight::Red
            }
        };
    counter = counter - 1;
    if counter==0 {
            panic!("over simulation");
        }
    }
}
