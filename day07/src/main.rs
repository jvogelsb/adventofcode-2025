use std::fs;

fn main() {
    let input = read_input("./input.txt");

    let total = part_one(input.as_str());
    println!("Part One Splits: {total}");
}

fn read_input(file_path: &str) -> String {
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    return input;
}

fn part_one(input: &str) -> i64 {
    let mut prev_row : Vec<char> = Vec::new();
    let mut num_splits : i64 = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut new_row: Vec<char> = vec!['.'; chars.len()];

        for (i, c) in chars.into_iter().enumerate() {
            let prev_is_pipe = matches!(prev_row.get(i), Some('|'));
            match c {
                'S' => new_row[i] = '|',
                '^' if prev_is_pipe => {
                    if let Some(prev) = i.checked_sub(1).and_then(|j| new_row.get_mut(j)) {
                        *prev = '|';
                    }
                    if let Some(next) = i.checked_add(1).and_then(|j| new_row.get_mut(j)) {
                        *next = '|';
                    }
                    new_row[i] = c;
                    num_splits += 1;
                }
                '.' if prev_is_pipe => {
                    new_row[i] = '|'
                },
                _ => {
                    if new_row[i] != '|' {new_row[i] = c}
                }
            }
        }
        // println!("{}", new_row.iter().collect::<String>());
        prev_row = new_row;
    }
    num_splits
}