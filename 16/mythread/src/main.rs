use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // threadの終了をまつ
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // moveクロージャで、メインスレッドの値を借用すn
    let handle = thread::spawn(move || {
        println!("v is {:?}", v);
    });

    handle.join().unwrap();
}
