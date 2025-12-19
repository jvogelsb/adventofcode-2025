use std::fs;

fn main() {
    let input = read_input("./input.txt");

    let pass = part_one(&input.0.as_str(), &input.1.as_str());
    println!("Part One Password: {pass}");

    let sum  = part_two(&input.0.as_str());
    println!("Part Two Sum: {sum}");
}

fn read_input(file_path: &str) -> (String, String) {
    let input = fs::read_to_string(file_path).expect("Failed to read file");

    let temp:Vec<&str> = input.split("\n\n").collect();
    println!("{:?}", temp);
    return (temp[0].to_string(), temp[1].to_string());
}

#[derive(Debug,Clone,Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn overlaps(&self, other:&Range) -> bool {
        return self.start <= other.start && self.end >= other.start ||
        self.start <= other.end && self.end >= other.end
    }

    fn merge(&mut self, other:&Range) {
        if other.start < self.start {
            self.start = other.start;
        }
        if other.end > self.end {
            self.end = other.end;
        }
    }
}

fn part_one(ranges: &str, ingredients: &str) -> i64 {
    let mut iranges: Vec<Range> = Vec::new();

    let mut num_fresh =0;
    for line in ranges.lines() {
        let temp:Vec<&str> = line.split("-").collect();
        iranges.push(Range{start: temp[0].parse().expect("Failed to parse start"), end: temp[1].parse().expect("Failed to parse end")});
    }

    for ing in ingredients.lines() {
        let ival:u64 = ing.parse().expect("Failed to parse ingredient");
        for irng in iranges.iter() {
            if ival >= irng.start && ival <= irng.end {
                num_fresh += 1;
                break;
            }
        }
    }
    num_fresh
}

fn part_two(ranges: &str) -> u64 {
    let mut iranges: Vec<Range> = Vec::new();

    for line in ranges.lines() {
        let temp:Vec<&str> = line.split("-").collect();
        iranges.push(Range{start: temp[0].parse().expect("Failed to parse start"), end: temp[1].parse().expect("Failed to parse end")});
    }

    iranges.sort_by(|a, b| {
        a.start
            .cmp(&b.start)
            .then(a.end.cmp(&b.end))
    });

    let mut final_ranges: Vec<Range> = Vec::new();
    for i in 0..iranges.len() -1 {  // next: Range (copied)
        let cur = iranges[i];
        let next = &mut iranges[i+1];
        if cur.overlaps(&next) {
            next.merge(&cur);
        } else {
            final_ranges.push(cur);
        }
    }
    final_ranges.push(*iranges.last().unwrap());
    let mut sum = 0;
    for rng in final_ranges {
        sum += rng.end - rng.start + 1;
    }
    sum
}