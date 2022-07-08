extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintlnで標準エラー出力
        eprintln!("引数解析に問題: {}", err);
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    println!("");

    // Config::newのエラーハンドリングと異なるが、こちらはエラーに関してのみ関心があり、返り値に関しては関心がない。
    // 返り値がある時→ unwrap系、返り値が無い時→if let で処理する？
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

