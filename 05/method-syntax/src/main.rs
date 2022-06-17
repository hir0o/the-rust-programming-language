struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hols(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

// 複数のimplブロック
impl Rectangle {
  // selfを受け取らない関数は、関連関数になる
  fn square(size: u32) -> Rectangle {
      Rectangle { width: size, height: size }
  }
 
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect1.area());
    println!("{}", rect1.can_hols(&rect2));
}
