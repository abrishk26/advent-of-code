use std::fs;
use std::io;

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn count_neighbors(grid: &[Vec<bool>], i: usize, j: usize) -> u32 {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    DIRECTIONS
        .iter()
        .filter(|&&(dr, dc)| {
            let nr = i as isize + dr;
            let nc = j as isize + dc;
            nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr as usize][nc as usize]
        })
        .count() as u32
}

fn find_valid_papers(grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell)
                .map(move |(j, _)| (i, j))
        })
        .filter(|&(i, j)| count_neighbors(&grid, i, j) < 4)
        .collect()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'@').collect())
        .collect();
    
    let mut output = 0;
    loop {
        let valid_papers = find_valid_papers(&grid);
        if valid_papers.is_empty() { break; }
        output += valid_papers.len();
        for (x, y) in valid_papers {
            grid[x][y] = false;
        }
    }
    
    println!("{output}");
    Ok(())
}
