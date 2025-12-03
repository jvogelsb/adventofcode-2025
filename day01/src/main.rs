use std::fs;

fn main() {
    println!("Hello, world!");
    let input = read_input("./input.txt");

    let mut pass = part_one(&input);
    println!("Part One Password: {pass}");

    pass = part_two(&input);
    println!("Part Two Password: {pass}");
}

fn read_input(file_path: &str) -> String {
    println!("File {file_path}");
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    return input;
}

// Returns number of times, dial stops at 0
fn part_one(input: &String) -> i32 {
    let mut pos: i32 = 50;
    let mut pass: i32 = 0;
    for line in input.lines() {
        let dir = &line[0..1];
        let dist = line[1..]
            .trim()
            .parse::<i32>()
            .expect("Failed to parse distance");
        print!("line: {line} Dir: {dir} dist: {dist}");
        (pos, _) = move_knob(dir, dist, pos);
        println!("   normalized pos: {pos}   pass; {pass}");
        if pos == 0 {
            pass += 1;
        }
    }
    return pass;
}

fn move_knob(dir: &str, dist: i32, pos: i32) -> (i32, i32) {
    let mut new_pos: i32;
    let mut num_passes: i32 = 0;
    if dir == "L" {
        new_pos = pos - dist;
        
        if pos == 0 {
            num_passes = dist / 100;
        } else if dist >= pos {
            num_passes = (dist - pos) / 100 + 1;
        }
        new_pos = pos - (dist % 100);
        if new_pos < 0 {
            new_pos += 100;
        }
    } else {
        new_pos = pos + dist;
        num_passes = new_pos / 100;
        new_pos %= 100;
    }
    return (new_pos, num_passes);
}

// // Returns number of times, dial passes 0
fn part_two(input: &String) -> i32 {
    let mut pos: i32 = 50;
    let mut pass: i32 = 0;
    let mut num_zeros: i32;
    for line in input.lines() {
        let dir = &line[0..1];
        let dist = line[1..]
            .trim()
            .parse::<i32>()
            .expect("Failed to parse distance");
        print!("line: {line} Dir: {dir} dist: {dist}");
        (pos, num_zeros) = move_knob(dir, dist, pos);
        pass += num_zeros;
        println!("   normalized pos: {pos}   pass; {pass}");
    }
    return pass;
}
