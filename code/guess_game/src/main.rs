use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guss number！");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please guess a number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("can not read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guess number is: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("you win!!");
                break;
            }
        }
    }
}