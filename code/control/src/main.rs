fn main() {
    // if
    if 1 == 2 {
        println!("whoops, mathematics broke");
    } else {
        println!("everything's fine!");
    }

    let formal = true;
    let greeting = if formal {
        "Good evening."
    } else {
        "Hello, friend!"
    };
    println!("{}", greeting); // prints "Good evening."

    //循环
    let mut i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        };
    };
    assert_eq!(something, 128);
    println!("{}", something);

    let mut counter = 0;
    
    while counter < 10 {
        println!("hello");
        counter = counter + 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for item in 0..5 {
        println!("{}", item * 2);
    }

    for item in (0..5).rev(){
        println!("{}", item)
    }
}