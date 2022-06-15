fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three!"),
        _ => (),
    }

    // if letはmatchの糖衣構文
    if let Some(3) = some_u8_value {
        println!("three!")
    }

    if some_u8_value == Some(3) {
        println!("three!")
    }

    // if letだと、マッチした値を参照できる
    if let Some(i) = some_u8_value {
        println!("{} is three!", i);
    }

    // これはエラー
    // if some_u8_value == Some(i) {
    //     println!("{} is three!", i);
    // }
    
    // 以下は等価
    match some_u8_value {
        Some(i) => println!("value is {}", i),
        None => println!("None!!"),
    }

    if let Some(i) = some_u8_value {
        println!("value is {}", i);
    } else {
        println!("None!!");
    }
}

