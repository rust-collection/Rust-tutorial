use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where 
T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let x = 1;

    // 闭包可以获得上下文中的变量，虽然该变量没有定义在闭包中
    let equal_to_x = move |z| z == x;
    println!("{}", x);
    let y = 4;

    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    // 甚至可以使用move来获得所有权
    let equal_to_x = move |z| z == x;

    // 因为x在这里已经没有所有权，所以会报错
    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

fn simulated_expansive_calculation(intensity: u32) -> u32 {
    println!("calcutlating slowly....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}