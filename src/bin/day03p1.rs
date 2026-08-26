use std::{fs::File};
use std::io::{BufRead, BufReader};

fn main() {
    let mut output = 0;
    let  file = File::open("input.txt").unwrap();
    let  reader = BufReader::new(file);

    for line in reader.lines() {
        let battery = line.unwrap()
            .bytes()
            .map(|b| (b - b'0') as i32)
            .collect::<Vec<i32>>();
        
        let mut max_index = 0;        

        for i in 1..(battery.len()-1) {
            if battery[i] > battery[max_index] { max_index = i; }
        }
        output += (battery[max_index as usize].to_string() + &battery[(max_index as usize + 1)..].iter().max().unwrap().to_string()).parse::<i32>().unwrap();
    }
    println!("{}", output);
}