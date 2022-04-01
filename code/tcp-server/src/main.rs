use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7788").unwrap();
    for stream in listener.incoming(){
        match stream{
            Ok(stream) =>{
                handle_connection(stream);
            },
            Err(e) => panic!("Error create listener {:?}", e),
        };
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{}",String::from_utf8_lossy(&buffer[..]))
}
