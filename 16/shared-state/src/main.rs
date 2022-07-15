use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        // ロックを取得してデータにアクセスする.
        // ロックを取得するまでスレッドをブロックする。
        // ロックを取得sいている他のスレッドがパニックしたらloaclの呼び出しは失敗する。
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // 複数のスレッドでMutex<T>を共有する
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 所有権を複数のスレッドに移すことはできないので、Rcで所有権をクローンする
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
