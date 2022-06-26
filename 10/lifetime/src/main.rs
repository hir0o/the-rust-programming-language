// ライフタイム: 参照が有効になるスコープのこと
fn main() {
    let string1 = String::from("abcd");
    
    let string2 = "xyx";

    let result = longest(string1.as_str(), string2);

    println!("longest strong is {}", result);
}

// 以下はエラーになる。
// 返り値が、xとyのどちらの参照なのかわからないから
// xとyの参照がわからないと、返り値の参照がどのくらいのライフタイムを持っているのかがわからない
// 戻り値のライフタイムがわからないとコンパイラが困っちゃう。囧
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// ライフタイム注釈をつける
// このことにより、引数の参照と、返り値の参照が同じライフタイムをもつことが保証される。
// 違うライフタイムのものを引数に入れることができない?
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

