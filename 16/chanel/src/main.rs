use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // tx: 送信側、rx: 受信側
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi!!");
        tx.send(val).unwrap();

        // sendしたら所有権がmoveするので、以下は許容されない。
        // println!("val is {}", val);
    });

    // 受信するまで待機
    let received = rx.recv().unwrap();

    println!("Get: {}", received);


    // 複数の値の送信
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Get: {}", receive);
    }
}
