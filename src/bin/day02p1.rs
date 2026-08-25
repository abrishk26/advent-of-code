use std::{fs::File, io::Read};

fn is_invalid_id(input: &str) -> bool {
    if input.len() % 2 != 0 {
        return false;
    }
    let (left, right) = input.split_at(input.len() / 2);
    left.parse::<i64>().unwrap() == right.parse::<i64>().unwrap()
}

fn main() {
    let mut output = 0;
    let mut file = File::open("input.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();

    for range in line.split(",") {
        let split = range.split("-").collect::<Vec<&str>>();
        let left = split[0].trim();
        let right = split[1].trim();
        for id in left.parse::<i64>().unwrap()..=right.parse::<i64>().unwrap() {
            if is_invalid_id(id.to_string().as_str()) {
                output += id;
            }
        }
    }
    println!("{}", output);
}
