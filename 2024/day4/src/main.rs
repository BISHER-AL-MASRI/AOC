use std::fs;
fn main() {
    let input = fs::read_to_string("src/input.input").unwrap();
    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let res = find_xmas(&grid);
    println!("{:?}", res);

}

fn find_xmas(grid: &[Vec<char>]) -> usize {
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (-1, -1), // Up-left
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
    ];
    let target = "XMAS".chars().collect::<Vec<char>>();
    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for (k, &ch) in target.iter().enumerate() {
                    let nx = i as isize + k as isize * dx;
                    let ny = j as isize + k as isize * dy;
                    if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize || grid[nx as usize][ny as usize] != ch {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}