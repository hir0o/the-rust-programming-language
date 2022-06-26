fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 上記をまとめたメソッド
// だが、これだとエラーが出る
// fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> T {
fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 構造体のジェネリック
struct Point<T> {
    x: T,
    y: T,
}

// メソッドのジェネリック
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 特定の型にのみ実装されるメソッド
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()

    }
}
// 特定のトレイト境界に実装されるメソッド
impl<T: Display + PartialOrd> Point {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest number is x = {}", self.x);
        } else {
            println!("largest number is y = ", self.y);
        }
    }
}
// 別のトレイトを実装するあらゆる型に対するトレイト実装...?
// ブランケット実装っていうらしい。
// Displayを実装してる型に対して、ToSoringトレイトを実装する例
impl<T: Display> ToString for T {

}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
