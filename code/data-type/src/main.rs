fn main() {
    let number: u32 = "42".parse().expect("Not a number!");
    println!("{}", number);

    let mut hello = String::from("hello, ");
    hello.push('w');
    hello.push_str("orld!");
    println!("{}", hello);

    let tuple = ("hello", 5, 'c');
    println!("{}", tuple.0);
}
