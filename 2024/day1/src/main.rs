use std::fs;
fn main() {
    let mut numbers: Vec<i32> = fs::read_to_string("src/input.input")
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut numbers2: Vec<i32> = fs::read_to_string("src/input.input")
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    // println!("{:?}", numbers);
    // println!("{:?}", numbers2);

    numbers.sort();
    numbers2.sort();

    let mut num = 0;

    for i in 0..numbers.len() {
        num += (numbers[i] - numbers2[i]).abs();
    }

    println!("{}", num);
}
