enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    // 使用宏，使得vector自动推断内部类型
    let mut v = vec![1,2,3];
    v.push(4);
    // 索引超出范围会panic
    let third: &i32 = &v[2];
    // 不能在一个作用域内对同一变量同时拥有可变借用和不可变借用
    // v.push(4);
    println!("third number is: {}", third);

    // get超出范围会返回None
    match v.get(2) {
        Some(third)=>println!("third number is: {}", third),
        None=>println!("there is no third number"),
    }

    for i in &mut v{
        *i += 50;
    }

    for i in &v{
        println!("{}", i);
    }

    // 通过附加值的枚举类型使得vector可以存储不同类型的值
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
