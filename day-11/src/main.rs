use memoize::memoize;
use std::fs;

type PuzzleInput = Vec<u64>;

fn read_input() -> PuzzleInput {
    let mut puzzle_input: PuzzleInput = vec![];
    let puzzle = fs::read_to_string("input.txt").unwrap_or("1 2024 1 0 9 9 2021976".to_string());
    for value in puzzle.split_whitespace() {
        puzzle_input.push(value.parse().unwrap());
    }
    puzzle_input
}

#[memoize]
fn blink(blinks_left: u64, rock: u64) -> u64 {
    if blinks_left == 0 {
        return 1;
    }
    if rock == 0 {
        return blink(blinks_left - 1, 1);
    } else {
        let rock_as_string = rock.to_string();
        if rock_as_string.len() % 2 == 0 {
            let (left, right) = rock_as_string.split_at(rock_as_string.len() / 2);
            let left: u64 = left.parse().unwrap();
            let right: u64 = right.parse().unwrap();
            return blink(blinks_left - 1, left) + blink(blinks_left - 1, right);
        } else {
            return blink(blinks_left - 1, rock * 2024);
        }
    }
}

fn solution(blinks: u64, puzzle: &PuzzleInput) -> u64 {
    let mut solution = 0;
    for rock in puzzle {
        solution += blink(blinks, *rock);
    }
    solution
}

fn main() {
    let puzzle = read_input();
    println!("Solution One: {}", solution(25, &puzzle));
    println!("Solution Two: {}", solution(75, &puzzle));
}
