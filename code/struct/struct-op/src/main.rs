struc User{
    username: String,
    email: String,
    sign_in_count: u64,
    activate: bool,
}

fn build_user(email: String, username:String)->User{
    let user1 = User{
        email,
        username,
        activate: false,
        sign_in_count: 10,
    };
    //结构体可以作为返回值进行返回
    User{
        // 当参数名与结构体字段名相同时可以使用简化赋值的方式对其进行赋值
        email,
        username,
        //当新声明的结构体与之前已经声明的结构值相同时，可以使用struct的更新操作
        ..user1,
    }
}

// tuple struct 需要注意的是虽然这两个元组结构体的内部元素相同但是他们的类型是不同的
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main() {
    println!("Hello, world!");

    let user1 = User{
        email: String::from("123@qq.com"),
        username: String::from("James"),
        activate: true,
        sign_in_count: 556,
    };

    println!("{}", user1.email);

    // struct的更新语法, 除了email和username外user2与user1的值相同
    let user2 = User{
        email: String::from("12345@qq.com"),
        username: String::from("James2"),
        ..user1,
    };

    let black = Color(0,0,0);
    let p = Point(0,0,0);
}