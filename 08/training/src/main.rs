use std::{collections::HashMap, io};


fn main() {
    vect();
    println!("{}", big_raten("apple"));
    hash_map_and_vec()
}

fn vect() {
    let numbers = vec![3, 4, 12, 55, 2, 19, 32, 78, 11, 2];
    println!("mean is {}", mean(&numbers));
    println!("mediam is {}", mediam(&numbers));
    println!("mode is {}", mode(&numbers));

}

// 平均を求める
fn mean(vec: &Vec<i32>) -> f64 {

    let mut total: f64 = 0.0;

    for n in vec {
        total += f64::from(*n);
    }

    return total / vec.len() as f64;
}

// ソートして真ん中
fn mediam(vec: &Vec<i32>) -> i32 {
    let mut vec = vec.to_vec();

    vec.sort();

    return *vec.get(vec.len() / 2).unwrap();
}

// 最も頻繁に出てくる値
fn mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for v in vec {
        let count = map.entry(v).or_insert(0);
        *count += 1;
    }

    let mut max = 0;

    for (_, val) in map {
        if val > max { max = val}
    }

    return max;
}

fn big_raten(word: &str) -> String {
   let mut s = String::from(word);
   let vowel = ["a", "i", "u", "e", "o"];
   match s.chars().nth(0).unwrap() {
       'a' | 'i' | 'u' | 'e' | 'o' => s.push_str("hay"),
       _ => {
           s.push(s.chars().nth(0).unwrap());
           s.remove(0);
           s.push_str("ay");
       }
   }
   s
}

fn hash_map_and_vec() {
    // ハッシュマップの作成
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    // コマンドの読み取り
    loop {
        println!("command >> ");

        let mut s = String::new();

        io::stdin().read_line(&mut s).expect("filed to read line");

        let command = s.trim().split(" ").collect::<Vec<&str>>();

        match command[0] {
            "Add" => {
                if command.len() != 4 || command[2] != "to" {
                    println!("{:?}", command);
                    println!("{}", "Add command is invalid");
                    continue;
                }
                let key = command[1];
                let value = command[3];
                if map.contains_key(key) {
                    let mut new_vec = map.get(key).unwrap().to_vec();
                    new_vec.push(value.to_string());
                    map.insert(key.to_string(), new_vec);
                } else {
                    map.insert(key.to_string(), vec![value.to_string()]);
                }
            },
            "Get" => {
                let key = command[1];
                if map.contains_key(key) {
                    println!("{:?}", map.get(key).unwrap());
                } else {
                    println!("not exist");
                }
            },
            "quit" => {
                break;
            }
            _ => {
                println!("unknown command");
                continue;
            }
        }
    }
}
