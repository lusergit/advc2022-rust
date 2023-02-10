use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
enum Kind {
    Rock,
    Paper,
    Scissors,
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn letter_to_kind(letter: &str) -> Result<Kind, String> {
    if letter == "A" || letter == "X" {
        return Ok(Kind::Rock);
    }
    if letter == "B" || letter == "Y" {
        return Ok(Kind::Paper);
    }
    if letter == "C" || letter == "Z" {
        return Ok(Kind::Scissors);
    }
    Err(format!("Non-existsing type {letter}"))
}

fn kind_score(k: Kind) -> u32 {
    match k {
        Kind::Rock => 1,
        Kind::Paper => 2,
        Kind::Scissors => 3,
    }
}

fn win_score(p1: Kind, p2: Kind) -> u32 {
    match p1 {
        Kind::Rock => match p2 {
            Kind::Rock => 3,
            Kind::Paper => 6,
            Kind::Scissors => 0,
        },
        Kind::Paper => match p2 {
            Kind::Rock => 0,
            Kind::Paper => 3,
            Kind::Scissors => 6,
        },
        Kind::Scissors => match p2 {
            Kind::Rock => 6,
            Kind::Paper => 0,
            Kind::Scissors => 3,
        },
    }
}

fn calc_move(p1: Kind, p2: Kind) -> Kind {
    match p1 {
        Kind::Rock => match p2 {
            Kind::Rock => Kind::Scissors,
            Kind::Paper => Kind::Rock,
            Kind::Scissors => Kind::Paper,
        },

        Kind::Paper => p2,

        Kind::Scissors => match p2 {
            Kind::Rock => Kind::Paper,
            Kind::Paper => Kind::Scissors,
            Kind::Scissors => Kind::Rock,
        },
    }
}

fn score(p1: Kind, p2: Kind) -> u32 {
    let p2move = calc_move(p1.clone(), p2);
    let kind_score = kind_score(p2move.clone());
    let win_score = win_score(p1, p2move);
    kind_score + win_score
}

fn parse_game(game: &str) -> u32 {
    let moves: Vec<Kind> = game
        .split(" ")
        .map(|move_| letter_to_kind(move_).unwrap())
        .collect();
    score(moves[0].clone(), moves[1].clone())
}

fn main() {
    let mut lines = read_lines("input".to_string());
    let mut total: u32 = 0;
    for line in lines {
        let bind = line.expect("unable to parse line");
        let line = bind.trim();
        let game = parse_game(line);
        total = total + game;
    }
    println!("Total: {total}");
}
