#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // 枚举变体存值
    Quarter(UsState),
}

fn  value_in_cents(coin: Coin) -> u8{
    // match匹配必须穷举所有的可能性
    // _可以用下划线来代表通配符来接受所有其他的可能性
    match coin {
        Coin::Penny =>{
            println!("penny");
            1
        }
        Coin::Nickel=>5,
        Coin::Dime=>10,
        // 将枚举变体中的值用state表示
        Coin::Quarter(state)=>{
            println!("state: {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None=>None,
        Some(i)=>Some(i +1),
    }
}

fn main() {
    let v = Some(0u8);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 只匹配一种可能性
    if let Some(3) = v{
        println!("three")
    }else{
        println!("others")
    }
}
