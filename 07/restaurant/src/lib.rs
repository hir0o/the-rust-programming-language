use std::fmt::Result;
use std::io::Result as IoResult; // asキーワードでリネームできる

// これが
use std::cmp::Ordering;
use std::io;
// こう
use std::{cmp::Ordering, io};

// これが
use std::io;
use std::io::Write;
// こう
use std::io::{self, Write};
// こう
use std::collections::*;

// 兄弟は、非公開でも参照できる
mod front_fo_house {
    struct Staff {
        name: String,
    }
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// useでモジュールをグローバルにする？
use crate::front_fo_house::hosting;

// use pub で、モジュールを外部に公開できる。(再公開)
// use pub crate::front_fo_house::hosting;

// 以下のようにしないのは習慣による。一つ親をuseで持ち込むことによって、関数がローカルに定義されていないことを明示できる。
// use crate::front_fo_house::hosting::add_to_waitlist;
// 一方で、構造体やenumその他の要素は、フルパスで書くのが習慣。はっきりとした理由ない。
use crate::front_fo_house::Staff;

pub fn eat_at_restaurant() {
    // 絶対パス
    // crate::front_fo_house::hosting::add_to_waitlist();
   
    // 相対パス(eat_at_restaurantと同じ階層からスタート)
    // front_fo_house::hosting::add_to_waitlist();
    
    // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // フルーツは知ることも変更することもできない。
    meal.seasonal_fruit = String::from("meron");

}

fn serve_order() {}

mod back_of_house {
    // 構造体を公開できるが、フィールドは個別に公開する必要がある。
    pub struct Breakfast {
        pub toast: String, // トーストが出ることはお役さんは知ってる
        seasonal_fruit: String, // どんなフルーツが出るかは知らない
    }

    // enumの中身は、全て公開される。
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        crate::serve_order();
    }

    fn cook_order() {}
}

