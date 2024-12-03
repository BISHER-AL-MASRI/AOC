// part 1
// use std::fs;
// use regex::Regex;
// fn main() {
//     let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
//     let mut sum = 0;
//     let input = fs::read_to_string("src/input.input").unwrap();
//     for cap in re.captures_iter(&input) {
//         let a = cap[1].parse::<i32>().unwrap();
//         let b = cap[2].parse::<i32>().unwrap();
//         sum += a * b;
//     }
//     println!("{}", sum);

// }


// part 2

use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.input").unwrap();

    // Compile the regex
    let re = Regex::new(r"(?P<instruction>do\(\)|don't\(\)|mul\(\s*(?P<a>\d+)\s*,\s*(?P<b>\d+)\s*\))").unwrap();

    let mut enabled = true; 
    let mut sum = 0;

    // Process each match
    for caps in re.captures_iter(&input) {
        let instruction = &caps["instruction"];
        
        if instruction == "do()" {
            enabled = true;
        } else if instruction == "don't()" {
            enabled = false;
        } else if enabled && caps.name("a").is_some() && caps.name("b").is_some() {
            let a: i32 = caps["a"].parse().unwrap();
            let b: i32 = caps["b"].parse().unwrap();
            sum += a * b;
        }
    }

    println!("Sum of enabled multiplications: {}", sum);
}
