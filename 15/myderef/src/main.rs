fn main() {
    let mut  x = 5;
    let y = &x;

    println!("x is {}", x);
    println!("y is {}", y);

    let mut x = 5;
    // xの値を指すボックスのインスタンス?????
    let y = Box::new(x);

    x = 6;

    println!("x is {}", x);
    println!("y is {}", *y);
}
