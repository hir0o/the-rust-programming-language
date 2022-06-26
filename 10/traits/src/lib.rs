// trait
// 特定の振る舞いをもつ、あらゆる型になりうることを指定できる
// インターフェースと似てる
//
// NewsArticle: ニュース記事
// Tweet: 280文字のツイート
// どちらも共通として、サマリーを表示したい

// traitの定義
// トレイトか対象の型が自分のクレートのlocalである時のみ、 型に対してトレイトを実装できる
// ので、pubをつける
pub trait Summary {
    fn summaraize_author(&self) -> String;
    // traitに実装を書くと、デフォルト実装になる
    fn summaraize(&self) -> String {
        format!("(Read more from {} ...)", self.summaraize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// traitの実装
impl Summary for NewsArticle {
    fn summaraize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// traitの実装
impl Summary for Tweet {
    // オーバーライド
    fn summaraize(&self) -> String {
        format!("1 new tweet from {}", self.summaraize_author())
    }
    fn summaraize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// traitを実装しているものを引数にとるメソッド
pub fn notify(item: &impl Summary) {
    println!!("Breaking news! {}", item.summaraize());
}
// トレイト境界構文
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summaraize());
}
// SummaryとDisplayを実装したものを受け取る
pub fn notify3(item: &(impl Summary + Display)) {
    //
}
// トレイと境界に対しても使える
pub fn notify4<T: Summary + Display>(item: &T) {
    //
}
// where句で指定できる
// これが
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    //
}
// こう
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where S: Display + Clone,
          U: Clone + Debug
{
    //
}


// トレイとを実装している型を返す。tweetを返しているが、使う側はそのことを知らない
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("aa"),
        content: String::from("aa"),
        reply: false,
        retweet: false,
    }
}


