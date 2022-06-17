fn main() {
    // 新規文字列の生成
    let mut s1 = String::new();
    // 初期値あり
    let mut s2 = String::from("hello");
    // to_stringでも作れる
    let data = "initial string";
    let s3 = data.to_string();
    let mut s4 = "initial string".to_string();


    // 文字列の更新
    s4.push_str(data);
    println!("s3 is {}", s4);
    s4.push('c'); // 文字列に追加

    // + 演算子、 format! マクロ
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();
    let s4 = s1 + "-" + &s2 + "-" + &s3;
    let s3 = format!("{} {}", s2, s3);
}
