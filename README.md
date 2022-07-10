# example_traffic_light
substrate入门第4课作业

为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同.


```rust
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
```

![](https://github.com/rustbomber/example_traffic_light/blob/9ba00e6d6fdd49f33598e3891405b3b68eaffd6d/screen.png)