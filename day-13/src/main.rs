use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Coords {
    y: i32,
    x: i32,
}

impl Coords {
    fn new() -> Coords {
        Coords { y: 0, x: 0 }
    }
}

#[derive(Debug)]
struct Puzzle {
    button_a: Coords,
    button_b: Coords,
    prize_location: Coords,
}

impl Puzzle {
    fn new() -> Puzzle {
        Puzzle {
            button_a: Coords::new(),
            button_b: Coords::new(),
            prize_location: Coords::new(),
        }
    }
}

fn get_coords_from_input(raw_input: &String) -> Coords {
    let regex = Regex::new(r"\d+").unwrap();
    let mut matches = regex.find_iter(&raw_input);
    let x: i32 = matches.next().unwrap().as_str().parse().unwrap();
    let y: i32 = matches.next().unwrap().as_str().parse().unwrap();
    Coords { y, x }
}

fn read_input() -> Vec<Puzzle> {
    let mut puzzles: Vec<Puzzle> = vec![];
    let raw_input = fs::read_to_string("input.txt").expect("Input missing");
    let mut current_puzzle: Puzzle = Puzzle::new();
    for line in raw_input.lines() {
        if line.is_empty() {
            puzzles.push(current_puzzle);
            current_puzzle = Puzzle::new();
        } else if line.starts_with("Button A:") {
            current_puzzle.button_a = get_coords_from_input(&line.to_string());
        } else if line.starts_with("Button B:") {
            current_puzzle.button_b = get_coords_from_input(&line.to_string());
        } else {
            current_puzzle.prize_location = get_coords_from_input(&line.to_string());
        }
    }
    puzzles.push(current_puzzle);
    puzzles
}

fn main() {
    let puzzles = read_input();
    for p in &puzzles {
        println!("{:?}", p);
    }
}
