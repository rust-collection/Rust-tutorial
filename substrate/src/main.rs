use std::f64::consts;
use std::time::Duration;

// 1.
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Show {
    fn remaining_time(&self) -> Duration;
}

impl Show for TrafficLight {
    fn remaining_time(&self) -> Duration {
        match self {
            TrafficLight::Red => Duration::from_secs(30),
            TrafficLight::Green => Duration::from_secs(20),
            TrafficLight::Yellow => Duration::from_secs(10),
        }
    }
}

// 2.
fn u32_sum(array: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in array {
        sum = match sum.checked_add(*i) {
            None => return Option::None,
            Some(sum) => sum,
        };
    }
    Option::Some(sum)
}

// 3.
trait Area {
    fn area(&self) -> f64;
}

fn get_area<T>(a: T) -> f64
where
    T: Area,
{
    a.area()
}

struct Circle {
    radius: f64,
}

struct Triangle {
    bottom_edge: f64,
    height: f64,
}

struct Rectangle {
    x: f64,
    y: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        (self.bottom_edge * self.height) / 2.0
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.x * self.y
    }
}

fn main() {
    let traffic_light = TrafficLight::Green;
    let time = traffic_light.remaining_time();
    println!("{}", time.as_secs());

    let traffic_light = TrafficLight::Red;
    let time = traffic_light.remaining_time();
    println!("{}", time.as_secs());

    let traffic_light = TrafficLight::Yellow;
    let time = traffic_light.remaining_time();
    println!("{}", time.as_secs());

    let array: &[u32] = &[3, 1, u32::MAX];
    match u32_sum(&array) {
        None => println!("None"),
        Some(sum) => println!("{}", sum),
    };

    let circle = Circle { radius: 3.0 };
    let triangle = Triangle {
        bottom_edge: 2.0,
        height: 4.0,
    };
    let rectangle = Rectangle { x: 1.0, y: 2.8 };
    println!("circle area {}", get_area(circle));
    println!("triangle area {}", get_area(triangle));
    println!("rectangle area {}", get_area(rectangle));
}
