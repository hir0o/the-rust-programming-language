extern crate iterators;

use iterators::Counter;

fn main() {
    let v1 = vec![1, 2, 3];

    let v1 = v1.iter();

    for val in v1 {
        println!("{}", val);
    }

    // イテレータを消費
    let v1 =  vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    println!("{:?}", v2);

    // 実装したイテレータ
    let counter = Counter::new();

    for val in counter {
        println!("count: {}", val);
    }

    let num: u32 = Counter::new().zip(Counter::new().skip(1))
                            .map(|(a, b)| a * b)
                            .filter(|x| x % 3 == 0)
                            .map(|x| x + 1)
                            .sum();

    println!("sum: {}", num);
}

