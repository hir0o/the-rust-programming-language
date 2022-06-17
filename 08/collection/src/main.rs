fn main() {
    // ベクタ
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    // マクロでベクタを作成。型はi32。
    let v2 = vec![1, 2, 3];

    // ベクタの値にアクセス
    // 1. 添字記法
    let third = &v[2];
    println!("therd element is {}", third);
    // 2. getメソッド
    let second = v.get(2); // Option<&T>
    match secon {
        Some(v) => println!("value is {}", v),
        None => println!("There is no third element."),
    }
    // 存在しない
    // let does_not_exist = &v[100]; // こっちはパニック
    // let does_not_exist = v.get(100); // こっちはNone
    
    // ベクタ要素をmapする
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i =+ 50;
    }

    // 複数の型を保持したい時は、emunを使う
    // tsのunionみたいなイメージ？
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("yaa")),
        SpreadsheetCell::Float(10.2),
    ]
}
