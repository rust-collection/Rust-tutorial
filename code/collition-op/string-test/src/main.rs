fn main() {
    // string无法被索引访问
    let mut s = "好".to_string();
    let s1 = String::from("hello");
    s.push_str(&s1);
    println!("{}", s);
    s.push('l');
    println!("{}", s);
    // 拼接两个字符串
    let s2 = s + &s1;
    // 拼接之后s不能继续使用，相当于调用add函数，函数会获取s所有权，然后将s1内容拼接进去
    // println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);

    let s3 = format!("{}-{}-{}", String::from("hello"), s1, s2);
    println!("{}", s3);

    println!("{}", s3.len());

    // 必须按照字符边界进行切片，否则会panic
    let s4 = &s3[0..5];
    println!("{}", s4);
}
