use num::complex::Complex;

fn main() {
    let number: u32 = "42".parse().expect("Not a number!");
    println!("{}", number);

    let mut hello = String::from("hello, ");
    hello.push('w');
    hello.push_str("orld!");
    println!("{}", hello);

    // 元组数据类型
    let tuple = ("hello", 5, 'c');
    println!("{}", tuple.0);

    // 数组数据类型
    // [数据类型;数组长度]
    let array:[i32;6] = [1,2,3,4,5,6];
    println!("{}", array[0]);

    // 相当于[3,3,3,3,3]
    let array = [3;5];
    println!("{}", array[2]);

    // 使用 num 包处理复数
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
