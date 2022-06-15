fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let res = match y {
        Some(i) => x + i,
        None => x,
    };

    println!("{}",res);

}
