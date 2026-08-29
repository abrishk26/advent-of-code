use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap();
    
    let is_separator: Vec<bool> = (0..width).map(|c| lines.iter().all(|l| l.as_bytes()[c] == b' ' )).collect();
    let mut fields: Vec<(usize, usize)> = Vec::new();
    let mut c = 0;
    while c < width {
        if is_separator[c] { c += 1; continue; }
        let start = c;
        while c < width && !is_separator[c] { c += 1;}

        fields.push((start, c));
    }

    let (operators_line, number_lines) = lines.split_last().unwrap();
    let operators: Vec<char> = operators_line.chars().filter(|c| !c.is_whitespace()).collect();
    
    let mut output = 0;
    for (i, &op) in operators.iter().enumerate() {
        let mut operands: Vec<&str> = Vec::new(); 
        let range = fields[i];
        for line in number_lines {
            let operand = &line[range.0..range.1];
            operands.push(operand);
        }
        let max_width = operands.iter().map(|o| o.len()).max().unwrap(); 
        let mut numbers: Vec<String> = vec![String::new(); max_width];
           for num in operands.iter() {
               for (col, digit) in num.chars().enumerate() {
                   numbers[col].push(digit); 
               }
        }
        output += match op {
            '+' => numbers.iter().map(|n| n.trim().parse::<u64>().unwrap()).sum::<u64>(),
            _ => numbers.iter().map(|n| n.trim().parse::<u64>().unwrap()).product()
        }
    }
    
    println!("{output}");
}