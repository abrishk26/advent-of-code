use std::{fs::File};
use std::io::{BufRead, BufReader};

fn main() {
    let mut output = 0;
    let  file = File::open("input.txt").unwrap();
    let  reader = BufReader::new(file);

    for line in reader.lines() {
        let battery = line.unwrap()
            .bytes()
            .map(|b| (b - b'0') as u64)
            .collect::<Vec<u64>>();

        let mut deletion_left = battery.len() - 12;
        let mut stack = Vec::with_capacity(battery.len());

        for digit in battery {
            while deletion_left > 0 && stack.last().is_some_and(|&last| digit > last)  {
               deletion_left -= 1; 
               stack.pop();
            } 

            stack.push(digit);
        }

        stack.truncate(12);
        output += stack.iter().fold(0, |acc, x | acc * 10 + x );
    }
    println!("{}", output);
}