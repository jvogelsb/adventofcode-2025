use std::fs;

fn main() {
    let input = read_input("./input.txt");

    let mut num_removed = part_one(&input);
    println!("Part One Removed: {num_removed}");

    num_removed = part_two(&input);
    println!("Part Two Removed: {num_removed}");
}

fn read_input(file_path: &str) -> String {
    println!("File {file_path}");
    let input = fs::read_to_string(file_path).expect("Failed to read file");
    input
}

fn part_one(input: &str) -> i64 {
    let map = build_map(input);
    count_accessible(&map)
}

fn build_map(input: &str) -> Vec<Vec<bool>> {
    let mut map: Vec<Vec<bool>> = vec![];
    for line in input.lines() {
        let mut row: Vec<bool> = vec![];
        for c in line.chars() {
            row.push(c == '@');
        }
        map.push(row);
    }
    map
}

fn print_map(map: &Vec<Vec<bool>>) {
    for row in map {
        for &cell in row {
            print!("{}", if cell { '@' } else { '.' });
        }
        println!();
    }
}

fn count_accessible(map: &Vec<Vec<bool>>) -> i64 {
    let mut num_accessible = 0;
    let rows = map.len();
    for (row_num, row) in map.iter().enumerate() {
        let cols = row.len();
        for col_num in 0..cols {
            if !map[row_num][col_num] {
                continue; 
            }
            if is_accessible(row_num, col_num, rows, cols, &map) {
                num_accessible += 1;
            }
        }
    }
    num_accessible
}


fn part_two(input: &str) -> i64 {
    let mut map = build_map(input);
    let mut total_removed = 0;
    loop {
        let num_removed = remove_accessible(&mut map);
        if num_removed == 0 {
            break;
        }
        total_removed += num_removed;
    }
    
    total_removed
}

fn remove_accessible(map: &mut Vec<Vec<bool>>) -> i64 {
    let mut num_removed = 0;
    let total_rows = map.len();
    let total_cols:usize = map[0].len();
    // for (row_num, row) in map.iter_mut().enumerate() {
    for i in 0..total_rows {
        let cols = map[i].len();
        for col_num in 0..cols {
            if !map[i][col_num] {
                continue; 
            }
            if is_accessible(i, col_num, total_rows, total_cols, map) {
                num_removed += 1;
                map[i][col_num] = false;
            }
        }
    }
    num_removed
}

fn is_accessible(row_num: usize, col_num : usize, num_rows: usize, num_cols : usize, map: &Vec<Vec<bool>>) -> bool {
    let mut surrounding = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let new_row: i64 = row_num as i64 + dr;
            let new_col = col_num as i64 + dc;
            if new_row >= 0
                && new_row < num_rows as i64
                && new_col >= 0
                && new_col < num_cols as i64
            {
                if map[new_row as usize][new_col as usize] {
                    surrounding += 1;
                }
            }
        }
    }

    surrounding < 4
}