fn main() {
    let mut a = 10;
    let b = true;
    println!("a is {}, b is {}", a, b);
    a = 20;
    println!("a is {}, b is {}", a, b);

    let number = 5;          // the first binding is created using the name "number"
    let number = number + 5; // a different binding shadows the name "number"
    let number = number * 2; // again, a new binding is created
    println!("The number is: {}", number);
}