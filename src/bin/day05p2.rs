use std::fs;

fn main()  {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in input.lines() {
        if line.is_empty() { break; }
        let (start, end) = line.split_once('-').unwrap();
        ranges.push((start.parse().unwrap(), end.parse().unwrap())); 
    }
    ranges.sort_unstable_by_key(|r| r.0);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for &(start, end) in &ranges[..] {
        match merged_ranges.last_mut() {
            Some(last) if start <= last.1 + 1 => last.1 = last.1.max(end), 
            _ => merged_ranges.push((start, end))
        }
    }
    let output: u64 = merged_ranges.iter().map(|(s, e)| e - s + 1).sum();
    println!("{output}");
}