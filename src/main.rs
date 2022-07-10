trait Light {
    fn get_interval(&self) -> i32;
}
enum TrafficLight {
    Red,
    Blue,
    Yellow,
}

impl Light for TrafficLight {
    fn get_interval(&self) -> i32 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Blue => 15,
            TrafficLight::Yellow => 5,
        }
    }
}

fn main() {
    let yellow_light = TrafficLight::Yellow;
    let mut interval = yellow_light.get_interval();
    assert_eq!(interval, 5);
    println!("yellow light interval: {}", interval);

    let red_light = TrafficLight::Red;
    interval = red_light.get_interval();
    assert_eq!(interval, 10);
    println!("red light interval: {}", interval);

    let blue_light = TrafficLight::Blue;
    interval = blue_light.get_interval();
    assert_eq!(interval, 15);
    println!("blue light interval: {}", interval);
}
