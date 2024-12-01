use std::fs;
fn main() {
    let (mut numbers, mut numbers2): (Vec<i32>, Vec<i32>) = fs::read_to_string("src/input.input")
        .unwrap()
        .lines()
        .map(|line| {
            let section: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
            (section[0], section[1])
        }).fold((vec![], vec![]), |(mut left, mut right), (l, r)| {
            left.push(l);
            right.push(r);
            (left, right)
        });

    // debug lines WHY IS RUST SO WIERD

    // println!("{}", numbers.len());

    // println!("{:?}", numbers);
    // println!("{:?}", numbers2);

    // UPDATE: IM CORRECT LETS OFGOOOOO

    numbers.sort();
    numbers2.sort();

    let mut num = 0;

    for i in 0..numbers.len() {
        num += (numbers[i] - numbers2[i]).abs();
    }

    println!("{}", num);
}
