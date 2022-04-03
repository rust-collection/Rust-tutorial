use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("loop {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("no loop {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    

    let v = vec![1,2,3];
    thread::spawn(move ||{
        for i in v{
            println!("{}", i);
        }
    });

    handle.join().unwrap();
}
