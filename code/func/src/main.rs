fn hello(){
    println!("hello");
}

fn func_a()-> i32{
    5
}

fn func_b(){
    let x = {
        let a = 1;
        a + 100
    };
    println!("{}", x); 
}

fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
     // If the divisor is zero, we want to return early with a `false` value
     if divisor == 0 {
        println!("{} % {} = {}", dividend, divisor, dividend);
    
        return false;
    } else {
        println!("{} % {} = {}", dividend, divisor, dividend % divisor);
        // No semi-colon after this return value for the function
        return dividend % divisor == 0;
    }
}

fn main() {
    println!("Hello, world!");
    hello();
    func_a();
    func_b();
    assert_eq!(is_divisible_by(2, 3), false);
    assert_eq!(is_divisible_by(5, 1), true);
    assert_eq!(is_divisible_by(17, 0), false);
    assert_eq!(is_divisible_by(24, 6), true);
}
