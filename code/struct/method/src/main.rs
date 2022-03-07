#[derive(Debug)]
struct Rectangle{
    width:u32,
    length:u32,
}

// 为结构体定义方法
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.length > other.length
    }

    // 定义关联函数(类似于静态函数)
    fn square(size :u32) -> Rectangle{
        Rectangle{
            width:size,
            length:size,
        }
    }
}

fn main() {
    // 调用关联函数
    let s = Rectangle::square(20);
    let rec1 = Rectangle{
        width:100,
        length:55,
    };
    let rec2 = Rectangle{
        width:30,
        length:50,
    };
    let rec3 = Rectangle{
        width:300,
        length:50,
    };
    println!("{}", rec1.area());
    // 使结构体输出更清晰
    println!("{:#?}", rec1);
    println!("{:#?}", s);

    println!("{}", rec1.can_hold(&rec2));
    println!("{}", rec1.can_hold(&rec3));
}