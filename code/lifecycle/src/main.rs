use std::fmt::Display;

fn longest_with_an_announcement<'a, T>
    (x:&'a str, y:&'a str, ann: T)->&'a str
where
    T:Display
{
    println!("ann {}", ann);
    if x.len() > y.len(){
        x
    }else {
        y
    }
}

fn main() {
    // let r;
    // {
    //     let x = 6;
    //     r = &x;
    // }
    //在此处调用r时，此时已经超出了x的生命周期，
    //r就会引用随机的空值，rust在编译器就防止出现这样的错误
    // println!("{}", r);

    let stirng1 = String::from("hello");
    let result;
    {
        let string2 = String::from("xyz");
        //下面就是在不同的生命周期下的例子
        result = longest(stirng1.as_str(), string2.as_str());
    }
    println!("longest string: {}", result);
}

// 通过声明周期标识符'a来表示不同参数的生明周期为多少
// 生命周期标注在&后面，使用空格将标注与类型分开
fn longest<'a>(x: &'a str, y: &'a str)->&'a str{
    if x.len() > y.len(){
        x
    }else {
        y
    }
}
