use std::collections::HashMap;

fn main() {
    // HashMapの作成
    // 1. HasnMap.newして、insertする
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 2. zipとcollectで作る

    let terms = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // ジェネリックに_を指定すると型を推論してくれる。
    let scores: HashMap<_, _> = terms.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);


    // ハッシュマップと所有権
    let field_name = "faborite color".to_string();
    let field_value = "blue".to_string();

    let mut map = HashMap::new();
    map.insert(field_name, field_value);


    // field_nameの所有権がうつってて使用できない
    // println!("{}", field_name);


    // 値にアクセス
    // 1. getメソッド
    let term_name = "Blue".to_string();
    let team_score = scores.get(&term_name); // Option<&V>
    match team_score {
        Some(v) => println!("{}", v),
        None => (),
    }

    // 2 forループ
    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    // 古い値がなければ値を入れる
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Red")).or_insert(50);

    // 古い値に基づいて値を更新
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count =  map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
