use std::fs;

fn main() {
    println!("Hello, world!");
    let input = read_input("./input.txt");

    let mut sum = part_one(&input);
    println!("Part One Sum: {sum}");

    sum = part_two(&input);
    println!("Part Two Sum: {sum}");
}

fn read_input(file_path: &str) -> String {
    println!("File {file_path}");
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    input
}

fn get_start_end(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split('-').collect();
    println!("Range 0: {}\t1: {}", parts[0], parts[1]);
    let start = parts[0].parse::<i64>().expect("invalid start number");
    let end = parts[1].parse::<i64>().expect("invalid end number");
    println!("Range: {} - {}", start, end);
    (start, end)
}

fn sum_ids(ids: Vec<i64>) -> i64 {
    let mut sum = 0;
    for id in ids {
        sum += id;
    }
    sum
}

fn part_one(input: &str) -> i64 {
    let mut invalid_ids: Vec<i64> = vec![];
    for rng in input.split(',') {
        let (start, end) = get_start_end(rng);
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
    let sum_one = sum_ids(invalid_ids);
    sum_one
}


fn part_two(input: &str) -> i64 {
    let mut invalid_ids: Vec<i64> = vec![];
    for rng in input.split(',') {
        let (start, end) = get_start_end(rng);
        for curr in start..=end {
            let string_curr: String = curr.to_string();
            let str_curr: &str = &string_curr.as_str();
            let l = str_curr.len();
            for i in (1..= l / 2).step_by(1) {
                if l % i != 0 {
                    continue;
                }
                let mut is_invalid = true;
                for j in (0..l).step_by(i) {
                    let t = &str_curr[j..j+i];
                    let r = &str_curr[..i];
                    if t != r {
                        is_invalid = false;
                        break;
                    }
                }
                if is_invalid {
                    invalid_ids.push(curr);
                     print!("\t{}", curr);
                    break;
                }
            }
        }
        println!("")
    }
    let sum_one = sum_ids(invalid_ids);
    sum_one
}
