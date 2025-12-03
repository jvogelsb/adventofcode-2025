use std::fs;

fn main() {
    println!("Hello, world!");
    let input = read_input("./example_input.txt");

    let mut sum = part_one(&input);
    println!("Part One Sum: {sum}");

    // sum = part_two(&input);
    // println!("Part Two Sum: {sum}");
}

fn read_input(file_path: &str) -> String {
    println!("File {file_path}");
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    input
}

fn parse_input(input: &String) -> Vec<i32> {
    let mut invalid_ids: Vec<i32> = vec![];
    for rng:Vec<&str> in input.split(',').collect() {
        let parts:Vec<&str> = rng.split('-').collect();
        let start = parts[0].parse::<i32>();
        let end = parts[1].parse::<i32>();
        for curr in start..=end {
            let str_curr = curr.to_string();
            let l = str_curr.len();
            if l % 2 != 0 {
                continue;
            }      
            let front_val = str_curr[0..l/2];
            let end_val = str_curr[l/2..];
            if front_val == end_val {
                invalid_ids.push(curr);
            }
        }
        
    }
    invalid_ids;
}

fn sum_ids(ids: Vec<i32>) -> i32 {
    let mut sum = 0;
    for id in ids {
        sum += id;
    }
    sum;
}

fn part_one(input: &String ) -> i32 {
    let invalid_ids = vec![];
    let invalid_ids = parse_input(input);
    let sum_one = sum_ids(invalid_ids);
    sum_one;
}