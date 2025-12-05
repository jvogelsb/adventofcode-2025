use std::fs;

fn main() {
    let input = read_input("./example_input.txt");

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

fn part_one(input: &str) -> i64 {
    let mut sum: i64 = 0;
    for curr in input.lines() {
        let string_curr: String = curr.to_string();
        let mut first = '0';
        let mut second = '0';
        let len = string_curr.len();
        for i in 0..len {
            let c = string_curr.chars().nth(i).unwrap();
            if i < len - 1 && c > first {
                first = c;
                second = string_curr.chars().nth(i + 1).unwrap();
            } else if c > second {
                second = c;
            }
        }
        // let temp = format!("{}{}", first, second);
        sum += format!("{}{}", first, second).parse::<i64>().unwrap();
    }
    sum
}

fn part_two(input: &str) -> i64 {
    println!("{}", input);
    0
}
