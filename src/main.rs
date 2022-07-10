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

    let red_light = TrafficLight::Red;
    interval = red_light.get_interval();
    assert_eq!(interval, 10);

    let blue_light = TrafficLight::Blue;
    interval = blue_light.get_interval();
    assert_eq!(interval, 15);
}
