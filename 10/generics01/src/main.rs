fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("largest: {}", largest(&number_list));
}

fn largest(list: &Vec<i32>) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
