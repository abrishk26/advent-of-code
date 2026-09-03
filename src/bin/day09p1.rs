use std::fs;
type Pos = (i64, i64);

fn get_area(first: Pos, second: Pos) -> i64 {
    let (x1, y1) = first;
    let (x2, y2) = second;

    let dx = x1 - x2 + 1;
    let dy = y1 - y2 + 1;

    dx.abs() * dy.abs()
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let coordinates: Vec<Pos> = input.lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(),y.parse().unwrap())
        }).collect();

    let output = coordinates.iter()
        .enumerate()
        .flat_map(|(i, &a)| coordinates[i+1..].iter().map(move |&b| get_area(a, b)))
        .max()
        .unwrap_or(0);
        
    println!("{output}");
}