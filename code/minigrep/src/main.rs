use minigrep::Config;
use std::env;
use std::process;

// 该程序为命令行程序，用于在指定文件搜索字符串
// 执行方式 cargo run query file
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
