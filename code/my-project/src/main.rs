use std::collections::HashMap
use std::fmt::Result;
// 为导入的包起别名
use std::io::Result as io_result;


// use std::io;
// use std::io::Write;
// use std::io::Result;
// 下面的代码等同于上面的代码
use std::io::{self, Write, Result};

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert(1,3);
}
