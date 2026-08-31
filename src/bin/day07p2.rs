use std::{fs};

fn dp(row: usize, col: usize, elements: &[&str], grid: &mut [Vec<Option<i64>>]) -> i64 {
    if col >= elements[0].len() { return 0; }
    if row == elements.len() { return 1; }

    if let Some(value) = grid[row][col] {
           return value;
    }
   
    let value = if elements[row].as_bytes()[col] == b'^' {
        dp(row + 1, col - 1, elements, grid) + dp(row + 1, col + 1, elements, grid)
    } else {
        dp(row + 1, col, elements, grid)
    };
   
    grid[row][col] = Some(value);
    value
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let (first, elements) = lines.split_first().unwrap();
    let entry_point = first.find('S').unwrap();
    let mut grid = vec![vec![None; first.len()]; elements.len() + 1];

    let output = dp(1, entry_point, elements, &mut grid);

    println!("{output}");
}
