use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(|x| uniq.insert(x))
}

fn main() {
    let line = read_lines("input".to_string()).next().unwrap().unwrap(); // There's just one line now
    let chars: Vec<char> = line.chars().collect();
    let mut counter = 0;
    let n = 14; // n = 4 for the first part
    for win in chars.windows(n) {
        if has_unique_elements(win) {
            println!("Unique starting at {}", counter + n);
            return;
        }
        counter = counter + 1;
    }
}
