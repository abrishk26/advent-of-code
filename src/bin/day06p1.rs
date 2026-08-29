use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let (operators_line, number_lines) = lines.split_last().unwrap();
    let operators: Vec<&str> = operators_line.split_whitespace().collect();
    let mut result: Vec<Option<u64>> = vec![None; operators.len()];

    for line in number_lines {
       let operands: Vec<&str> = line.split(' ').filter(|o| !o.is_empty()).collect(); 
       for (i, op) in operands.iter().enumerate() {
           let value: u64 = op.parse().unwrap();
           result[i] = Some(match (result[i], operators[i]) {
               (None, _) => value,
               (Some(acc), "+") => acc + value,
               (Some(acc), _) => acc * value
           });
       }
    }

    let output: u64 = result.iter().map(|r| r.unwrap_or(0)).sum();
    println!("{output}");
}