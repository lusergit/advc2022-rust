use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let fname = filename.to_string();
    let file = File::open(fname).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn parse_instr(instr: String) -> (usize, usize, usize) {
    let stripped = instr
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "");
    let v: Vec<usize> = stripped
        .split(" ")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    // -1 to get directly indexes
    (v[0], v[1] - 1, v[2] - 1)
}

fn main() {
    let crates = read_lines("crates");
    let mut vec: Vec<Vec<char>> = Vec::new();
    for cratee in crates {
        let bind = cratee.unwrap();
        vec.push(bind.split(" ").map(|c| c.chars().next().unwrap()).collect());
    }
    let instructions = read_lines("input");
    for instr in instructions {
        let bind = instr.unwrap();
        let (n, from, to) = parse_instr(bind);
        let mut tmp: Vec<char> = Vec::new();
        for i in 0..n {
            let popped = vec[from].pop().unwrap();
            tmp.push(popped);
        }
        for i in 0..n {
            let popped = tmp.pop().unwrap();
            vec[to].push(popped);
        }
    }

    for v in vec {
        print!("{}", v.last().unwrap());
    }
}
