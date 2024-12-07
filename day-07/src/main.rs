use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct PuzzleInput {
    value_to_make: i64,
    values_to_use: Vec<i64>,
}

fn parse_line_from_input(line: String) -> PuzzleInput {
    let line: Vec<&str> = line.split(": ").collect();
    let values: Vec<i64> = line[1]
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();
    PuzzleInput {
        value_to_make: line[0].parse().unwrap(),
        values_to_use: values,
    }
}

fn read_input() -> Vec<PuzzleInput> {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let puzzle_inputs: Vec<PuzzleInput> = buf
        .lines()
        .map(|l| parse_line_from_input(l.expect("Could not parse line")))
        .collect();
    puzzle_inputs
}

fn has_solution_part_one(puzzle_input: &PuzzleInput) -> bool {
    search_solution_part_one(&puzzle_input, 1, puzzle_input.values_to_use[0])
}

fn has_solution_part_two(puzzle_input: &PuzzleInput) -> bool {
    search_solution_part_two(&puzzle_input, 1, puzzle_input.values_to_use[0])
}

fn search_solution_part_one(puzzle_input: &PuzzleInput, index: usize, accumulator: i64) -> bool {
    if index == puzzle_input.values_to_use.len() {
        return accumulator == puzzle_input.value_to_make;
    }

    let n = puzzle_input.values_to_use[index];
    search_solution_part_one(&puzzle_input, index + 1, n + accumulator)
        || search_solution_part_one(&puzzle_input, index + 1, n * accumulator)
}

fn concat(left: i64, right: i64) -> i64 {
    let mut shift = 1;
    while shift <= right {
        shift *= 10;
    }

    (left * shift) + right
}

fn search_solution_part_two(puzzle_input: &PuzzleInput, index: usize, accumulator: i64) -> bool {
    if index == puzzle_input.values_to_use.len() {
        return accumulator == puzzle_input.value_to_make;
    }

    let n = puzzle_input.values_to_use[index];
    search_solution_part_two(&puzzle_input, index + 1, n + accumulator)
        || search_solution_part_two(&puzzle_input, index + 1, n * accumulator)
        || search_solution_part_two(&puzzle_input, index + 1, concat(accumulator, n))
}

fn solution_one(puzzle_inputs: &Vec<PuzzleInput>) -> i64 {
    let mut n_with_solution = 0;
    for input in puzzle_inputs {
        if has_solution_part_one(&input) {
            n_with_solution += input.value_to_make;
        }
    }
    n_with_solution
}

fn solution_two(puzzle_inputs: &Vec<PuzzleInput>) -> i64 {
    let mut n_with_solution = 0;
    for input in puzzle_inputs {
        if has_solution_part_two(&input) {
            n_with_solution += input.value_to_make;
        }
    }
    n_with_solution
}

fn main() {
    let puzzle_inputs = read_input();
    println!("Solution One: {}", solution_one(&puzzle_inputs));
    println!("Solution Two: {}", solution_two(&puzzle_inputs));
}
