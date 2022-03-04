fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
    // rust中防止double free的方法使如果有新的s1指向与s相同的heap空间，则s会失效

    // let s1 = String::from("hello");
    // let s2 = s1;
    // borrow of moved value: `s1`
    // value borrowed here after moverustc(E0382)
    // main.rs(6, 9): move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
    // main.rs(7, 14): value moved here
    // main.rs(8, 20): value borrowed here after move
    // println!("{}", s1)

    take_ownership(s);
    // 同样会发生所有权转移的move操作，下面的语句会报错
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    // 调用该函数后会使s2所有权转移
    // 函数返回后将所有权转移给s3
    // 如果不发生所有权转移，那么当heap的变量离开作用域时，他的值就会被drop函数处理
    // 如果数据的所有权发生转移，那么就不会被清理
    let s3 = takes_and_gives_back(s2);

    // 让某个函数使用值但不获得其所有权
    // 下面三行的实现方式就会转移string的所有权
    let string = String::from("hello");
    let (string2, len) = calculate_length(string);
    println!("The length of '{}' is {}.", string2, len);

    // 通过引用的方式来防止值所有权转移
    let string = String::from("hello");
    let len = calculate_length1(&string);
    println!("The length of '{}' is {}.", string, len);

    // 如果使用可以修改的引用方式则可以在函数中对值进行修改
    let mut s1 = String::from("hello");
    // 在特定作用域内只能有一个可变的引用
    // 或者多个不可变引用
    // 以上两个条件只能满足一个
    let s2 = &mut s1;
    // 此时下方的语句就会报错，因为rust只允许可变变量拥有一个可变引用
    // 通过这种方式来防止数据竞争，当s2和s3同时对数据发生修改时在其他语言就可能会发生错误
    // let s3 = &mut s1;
    let len1 = calculate_length2(s2);
    // let len2 = calculate_length2(s3);
    println!("The length of '{}' is {}.", string, len1);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // 因为s在函数上已经声明了不可变的引用
    // cleaer函数尝试将其声明为不可变的引用，此时rust就会报错
    // s.clear();
    println!("{}", word);
}

// 将参数转换为字符串切片会使得函数更加灵活，可以同时接受字符串与字符串切片 
// 字符串传递参数的时候直接传递 let word = first_word(&s[..]); 即可
// fn first_word(s: &str) -> &str{
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    // iter会返回遍历集合中的每个元素
    // enumrate会将每一个元素添加一个下标并封装为元组进行返回
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            // 返回字符串的切片
            return &s[.. i];
        }
    }
    &s[..]
}

// 下面的函数是没办法通过编译的
// 因为函数题内声明了变量s，但是函数结束s的作用域已经失效就会被drop掉
// 但是却想返回s的引用，也就是野指针问题，rust直接不让这种情况通过编译
// fn dangle() -> &String{
//     let s = String::from("hello");
//     &s
// }

fn calculate_length2(s:&mut String)->usize{
    // 可变引用
    s.push_str(", world");
    s.len()
}

fn calculate_length1(s:&String)->usize{
    // 在该函数中不可以s属于借用过来的，所以不能修改其的值
    s.len()
}

fn calculate_length(s:String)->(String, usize){
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String{
    let some_string = String::from("from");
    // 函数返回后some_string会脱离作用域，将所有权交给s1
    some_string
}

fn takes_and_gives_back(a_string:String) ->String{
    a_string
}

fn take_ownership(some_string: String){
    println!("{}",some_string);
}

fn makes_copy(some_number:i32){
    println!("{}", some_number)
}