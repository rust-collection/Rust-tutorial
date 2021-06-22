// 使结构体派生于Debug，使其可以格式化输出
#[derive(Debug)]
struct Rectangle{
    width:u32,
    length:u32,
}

fn area(rec: &Rectangle) -> u32{
    rec.width * rec.length
}

fn main() {
    let rec = Rectangle{
        width:30,
        length:50,
    };
    println!("{}", area(&rec));
    // 使结构体输出更清晰
    println!("{:#?}", rec)
}
