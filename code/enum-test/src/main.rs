// 枚举类型的变体
#[derive(Debug)]
enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message{
    Quit,
    // 存储匿名的结构体
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){}
}

fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let q = Message::Quit;
    let m = Message::Move {x:12,y:12};
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(0, 255,255);
    m.call();

    // Rust中没有null，但是有与null概念相似的枚举Option<T>
    // 它包含在预导入模块中，可以直接使用
    // enum Option<T>{
    //     Some<T>,
    //     Node,
    // }
    let some_number = Some(5);
    let some_string = Some("String");

    // 代表空值
    let absent_number: Option<i32> = None;
    // Option<T>与T是不同类型的，不可以把Option<T>直接当成T来操作
    // 如果想使用Option<T>中的T，必须将它转换为T
}
