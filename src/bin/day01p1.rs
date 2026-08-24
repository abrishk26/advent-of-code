use std::fs;
use std::io::{BufRead, BufReader};

enum Direction {
    Left,
    Right,
}

fn parse_line(mut line: String) -> (Direction, i32) {
    let rotation = line.split_off(1).trim().to_owned();
    if line == "R" {
        return (Direction::Right, rotation.parse().unwrap());
    }
    if line == "L" {
        return (Direction::Left, rotation.parse::<i32>().unwrap());
    }

    unreachable!()
}

fn main() {
    let f = fs::File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut dial_point = 51;
    let mut password = 0;
    while let Ok(c) = reader.read_line(&mut line) {
        if c == 0 { break; }
        let (direction, rotation) = parse_line(line.clone());
        dial_point = calculate_dial(direction, dial_point, rotation % 100);
        if dial_point == 1 { password += 1; }
        line.clear();
    }

    println!("{password}");
}

fn calculate_dial(direction: Direction, dial_point: i32, rotation: i32) -> i32 {
    let rot = match direction {
        Direction::Left => dial_point - rotation,
        Direction::Right => dial_point + rotation,
    };

    if rot < 0 {
        return 100 + rot;
    }

    rot % 100
}
