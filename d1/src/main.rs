use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn main() {
    let lines = read_lines("input".to_string());
    let mut elves: Vec<i32> = Vec::new();
    let mut cals: Vec<i32> = Vec::new();
    for line in lines {
        let lline = line.expect("could not read input");
        let trimmed = lline.trim();
        if trimmed != "" {
            let val = trimmed.parse::<i32>().expect("Could not parse value");
            cals.push(val);
        } else {
            let sum = cals.iter().sum();
            elves.push(sum);
            cals.clear();
        }
    }
    elves.sort();
    let threes = elves.iter().rev().take(3);
    let sum: i32 = threes.sum();
    println!("Sum: {sum}");
}
