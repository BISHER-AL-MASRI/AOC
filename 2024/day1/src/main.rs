// part 1

use std::fs;
fn main() {
    let (mut leftVector, mut rightVector): (Vec<i32>, Vec<i32>) = fs::read_to_string("src/input.input")
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

    // println!("{}", leftVector.len());

    // println!("{:?}", leftVector);
    // println!("{:?}", rightVector);

    // UPDATE: IM CORRECT LETS OFGOOOOO

    leftVector.sort();
    rightVector.sort();

    let mut num = 0;

    for i in 0..leftVector.len() {
        num += (leftVector[i] - rightVector[i]).abs();
    }

    println!("{}", num);
}

// part 2

// use std::{collections::HashMap, fs};

// fn main() {
//     let (left_vector, right_vector): (Vec<i32>, Vec<i32>) = fs::read_to_string("src/input.input")
//         .unwrap()
//         .lines()
//         .map(|line| {
//             let section: Vec<i32> = line
//             .split_whitespace()
//             .filter_map(|s| s.parse::<i32>().ok())
//             .collect();
//             (section[0], section[1])
//         }).fold((vec![], vec![]), |(mut left, mut right), (l, r)| {
//             left.push(l);
//             right.push(r);
//             (left, right)
//         });
    

//     let mut frequency_map = HashMap::new();

//     for &num in &right_vector {
//         *frequency_map.entry(num).or_insert(0) += 1;
//     }

//     let mut sim = 0;
//     for &num in &left_vector {
//         let freq = frequency_map.get(&num).unwrap_or(&0);
//         sim += num * freq;
//     }

//     println!("{}", sim);
// }




