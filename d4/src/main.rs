use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

struct Section {
    min: u32,
    max: u32,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlap(&self, other: &Section) -> bool {
        (self.min <= other.min && self.max >= other.max)
            || (other.min >= self.min && other.min <= self.max)
            || (other.max >= self.min && other.max <= self.max)
    }

    fn new(min: u32, max: u32) -> Section {
        Section { min, max }
    }
}

fn parse_line(line: &String) -> Vec<Section> {
    line.split(",")
        .map(|sec_str| {
            sec_str
                .split("-")
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .map(|v: Vec<u32>| Section::new(v[0], v[1]))
        .collect()
}

fn main() {
    let lines = read_lines("input".to_string());
    let mut counter = 0;
    for line in lines {
        let bind = line.unwrap();
        let elves: Vec<Section> = parse_line(&bind);
        if elves[0].overlap(&elves[1]) || elves[1].overlap(&elves[0]) {
            counter = counter + 1;
        }
    }
    println!("Elves overlapping: {counter}")
}
