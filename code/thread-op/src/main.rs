use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


fn channel_ex() {
    let (tx, rx) = mpsc::channel();
    
    publish_msg(tx, 1);
    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
}

fn thread_test(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("loop {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("no loop {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    

    let v = vec![1,2,3];
    thread::spawn(move ||{
        for i in v{
            println!("{}", i);
        }
    });

    handle.join().unwrap();
}


fn publish_msg(tx : mpsc::Sender<i32>, message : i32){
    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(message).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });
}