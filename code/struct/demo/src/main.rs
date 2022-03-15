// 使结构体派生于Debug，使其可以格式化输出
#[derive(Debug)]
struct Rectangle{
    width:u32,
    length:u32,
}

//定义泛型结构体
struct Point<T>{
    x:T,
    y:T,
}

//impl表示针对泛型T实现的方法
impl<T> Point<T>{
    fn x(&self)->&T{
        &self.x
    }
}

//下面定义的方法只有在参数类型为i32时才有x1方法
impl Point<i32>{
    fn x1(&self)->&i32{
        &self.x
    }
}

//多泛型例子
struct Point1<T, U>{
    x:T,
    y:U,
}

impl<T,U> Point1<T,U>{
    fn mixup<V,W>(self, other:Point1<V,W>)->Point1<T,W>{
        Point1{
            x:self.x,
            y:other.y,
        }
    }
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
    println!("{:#?}", rec);

    let a = Point{x:5,y:10};
    let b = Point{x:1.0,y:3.5};
}
