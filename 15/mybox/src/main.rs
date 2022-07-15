use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    // Derefトレイトが使用する関連型を定義
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
