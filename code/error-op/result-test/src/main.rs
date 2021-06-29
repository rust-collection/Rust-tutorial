use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn read_username_from_file(path:&str) -> Result<String, io::Error>{
    let f = File::open(path);
    let mut f = match f {
        Ok(file)=>file,
        Err(e)=>return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let f = File::open("hello.txt");

    // 调用unwarp方法如果成功返回file，错误调用err
    let f = File::open("hello.txt").unwrap();
    // 与unwarp类似但是错误消息可自定义
    let f = File::open("hello.txt").expect("cannot open the file");

    let f = match f {
        Ok(file)=>file,
        Err(error)=> match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) =>fc,
                Err(e) =>panic!("Error creating file {:?}", e),
            },
            other_err=>panic!("Error opening file: {:?}", other_err),
        },
    };

    // 下面的代码等同于上面的代码
    let f = File::open("hello.txt").unwarp_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwarp_or_else(|error|{
                panic!("Error creating file {:?}", error)
            })
        }else{
            panic!("Error opening file: {:?}", error);
        }
    });

    let result = read_username_from_file("hello.txt");
}
