use std::fs;

// part 1
// i cant explain this code since i dont even know what i did. rust is complicated
fn main() {
    let input = fs::read_to_string("src/input.input")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut safe_reports = 0;

    for report in input {
        if is_safe_report(&report) {
            safe_reports += 1;
        }
    }

    println!("Number of safe reports: {}", safe_reports);
}

fn is_safe_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true; 
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];

        if diff < -3 || diff > 3 || diff == 0 {
            return false;
        }

        if diff > 0 {
            decreasing = false;
        } else if diff < 0 {
            increasing = false;
        }
    }

    increasing || decreasing
}

