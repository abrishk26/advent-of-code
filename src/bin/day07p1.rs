use std::{fs, collections::HashSet};

fn main()  {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect(); 
    let (first, elements) = lines.split_first().unwrap();
    let mut split_points = HashSet::from([first.find('S').unwrap()]);

    let mut output = 0;
    for line in elements {
        let mut points = HashSet::new();
        for &i in &split_points {
            if line.as_bytes()[i] == b'^' {
                output += 1;
                if i > 0 { points.insert(i - 1); }
                if i + 1 < line.len() { points.insert(i + 1); }
            } else { points.insert(i); }
        } 

        split_points = points;
    }

    println!("{output}");
}