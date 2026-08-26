use std::{fs::File, io::Read};

fn is_invalid_id(input: &str) -> bool {
    let len = input.len();
    for i in 1..=(len / 2) {
        if len % i != 0 {
            continue;
        }

        let block = &input[..i];
        if input.as_bytes().chunks(i).all(|c| c == block.as_bytes()) { return true; };
    }
    false
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
            let result = is_invalid_id(id.to_string().as_str());
            if result {
                output += id;
            }
        }
    }
    println!("{}", output);
}
