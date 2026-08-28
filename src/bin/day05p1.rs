use std::fs;

fn main()  {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut is_ranges = true;
    let mut ids: Vec<u64> = Vec::new();
    for line in input.lines() {
        if line.is_empty() { is_ranges = false; continue; }
        if is_ranges {
            let (start, end) = line.split_once('-').unwrap();
            ranges.push((start.parse().unwrap(), end.parse().unwrap())); 
        } else {
           ids.push(line.parse().unwrap()); 
        }
    }

    let output = ids
        .iter()
        .filter(|&&id| ranges.iter().any(|&(l, r)| (l..=r).contains(&id) ))
        .count();
    
    println!("{output}");
}