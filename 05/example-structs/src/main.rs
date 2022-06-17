fn main1() {
    let width1 = 30;
    let height1 = 50;

    println!("長方形の面積は、{}平方ピクセルです", area(width1, height1))
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// tupleでリファクタ
fn main2() {
    let rect1 = (30, 50);


    println!("長方形の面積は、{}平方ピクセルです", area2(rect1))
}

fn area2(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

// 構造体でリファクタリング
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 借用しないと、関数を呼び出した後にrect1が破棄されてしまう
    println!("長方形の面積は、{}平方ピクセルです", area3(&rect1));

    println!("{:?}", rect1);
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
