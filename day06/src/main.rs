use std::fs;

fn main() {
    let input = read_input("./example_input.txt");

    let total = part_one(input);
    println!("Part sum: {total}");
}

fn read_input(file_path: &str) -> String {
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    return input;
}

fn part_one(input: String) -> i64 {
    let mut total: i64 = 0;
    let lines:Vec<&str> = input.lines().collect();
    let len = lines.len();
    let operations: Vec<&str> = lines[len - 1].split(" ").filter(|s| !s.is_empty()).collect();
    for i in 0..len-1 {
        let operands: Vec<&str> = lines[i].split(" ").filter(|s| !s.is_empty()).collect();
        let val = do_operation(operations[i], operands);
        total += val;
    }
    total
}

fn do_operation(operator: &str, operands: Vec<&str>) -> i64 {
    let mut total: i64 = 0;
    if operator == "+" {
        for operand in operands.iter() {
            let v = operand.parse::<i64>().expect("failed to parse number");
            total += v;
        }
    } else {
        total = 1;
       
        for operand in operands.iter() {
            let v = operand.parse::<i64>().expect("failed to parse number");
            total *= v;
        }
    }
    println!("{} {:?} = {}", operator, operands, total);
    total
}
