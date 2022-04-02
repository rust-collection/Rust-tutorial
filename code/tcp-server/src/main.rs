// 导入需要的模块
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // 创建 tcp server 监听在7788端口
    let listener = TcpListener::bind("127.0.0.1:7788").unwrap();
    // 收取客户端发送过来的消息
    for stream in listener.incoming(){
        // 信息传输过程中可能会出现问题
        // 所以 stream 是Result类型的
        match stream{
            // 当正确收到消息时通过 handle_connection 函数处理消息
            Ok(stream) =>{
                handle_connection(stream);
            },
            // 错误的话引发 panic
            Err(e) => panic!("Error create listener {:?}", e),
        };
    }
}

// 定义处理消息的函数
fn handle_connection(mut stream: TcpStream){
    // 定义 buffer 用于存储接收到的消息
    let mut buffer = [0; 512];
    // 通过 read 方法将消息读取到buffer中
    stream.read(&mut buffer).unwrap();
    // 将消息输出
    println!("{}",String::from_utf8_lossy(&buffer[..]))
}
