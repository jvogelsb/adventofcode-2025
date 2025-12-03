use std::fs;

fn main() {
    println!("Hello, world!");
    let input = read_input("./input.txt");

    let sum = part_one(&input);
    println!("Part One Sum: {sum}");

    // sum = part_two(&input);
    // println!("Part Two Sum: {sum}");
}

fn read_input(file_path: &str) -> String {
    println!("File {file_path}");
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    input
}

fn parse_input(input: &str) -> Vec<i64> {
    let mut invalid_ids: Vec<i64> = vec![];

    for rng in input.split(',') {
        let parts: Vec<&str> = rng.split('-').collect();
        println!("Range 0: {}\t1: {}", parts[0], parts[1]);
        let start = parts[0].parse::<i64>().expect("invalid start number");
        let end = parts[1].parse::<i64>().expect("invalid end number");
        println!("Range: {} - {}", start, end);
        for curr in start..=end {
            let string_curr: String = curr.to_string();
            let str_curr: &str = &string_curr.as_str();
            let l = str_curr.len();
            if l % 2 != 0 {
                continue;
            }
            let front_val = &str_curr[0..l / 2];
            let end_val = &str_curr[l / 2..];
            if front_val == end_val {
                invalid_ids.push(curr);
                print!("\t{}", curr)
            }
        }
        println!("")
    }
    invalid_ids
}

fn sum_ids(ids: Vec<i64>) -> i64 {
    let mut sum = 0;
    for id in ids {
        sum += id;
    }
    sum
}

fn part_one(input: &str) -> i64 {
    let invalid_ids = parse_input(input);
    let sum_one = sum_ids(invalid_ids);
    sum_one
}
