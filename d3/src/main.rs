use array_tool::vec::Intersect;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn shared_elems(s1: String, s2: String, s3: String) -> Vec<char> {
    let mut s1_chars: Vec<char> = s1.chars().collect();
    let mut s2_chars: Vec<char> = s2.chars().collect();
    let mut s3_chars: Vec<char> = s3.chars().collect();

    let i1 = s1_chars.clone().intersect(s2_chars.clone());
    let i2 = s1_chars.intersect(s3_chars.clone());
    let i3 = s2_chars.intersect(s3_chars.clone());

    i1.intersect(i2.intersect(i3))
}

fn score(c: char) -> u32 {
    let buff = if c.is_ascii_lowercase() {
        96 // 97 - 1
    } else {
        38 // 41 - 27
    };
    let score = (c as u32) - buff;
    println!("char {} is getting score {}", c, score.clone());
    score
}

fn main() {
    let mut lines = read_lines("input".to_string());
    let mut scores: Vec<u32> = Vec::new();
    while let Some(line1) = lines.next() {
        let mut line2 = lines.next().unwrap();
        let mut line3 = lines.next().unwrap();

        let shared = shared_elems(line1.unwrap(), line2.unwrap(), line3.unwrap()); // Serve tra le tre linee

        scores.push(shared.iter().map(|c| score(*c)).sum());
    }
    println!("Score: {}", scores.iter().sum::<u32>())
}
