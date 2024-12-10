use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

type PuzzleInput = Vec<Vec<Option<u32>>>;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coords {
    y: i32,
    x: i32,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn value(&self) -> Coords {
        match &self {
            Direction::Up => Coords { y: -1, x: 0 },
            Direction::Right => Coords { y: 0, x: 1 },
            Direction::Down => Coords { y: 1, x: 0 },
            Direction::Left => Coords { y: 0, x: -1 },
        }
    }
}

impl std::ops::Add<&Direction> for &Coords {
    type Output = Coords;

    fn add(self, dir: &Direction) -> Coords {
        let coord_to_add = dir.value();
        Coords {
            x: self.x + coord_to_add.x,
            y: self.y + coord_to_add.y,
        }
    }
}

fn read_input() -> PuzzleInput {
    let file = File::open("input.txt");
    match file {
        Ok(f) => {
            let buf = BufReader::new(f);
            let grid: PuzzleInput = buf
                .lines()
                .map(|l| {
                    l.expect("Could not parse line")
                        .chars()
                        .map(|c| c.to_digit(10))
                        .collect()
                })
                .collect();
            return grid;
        }
        Err(_) => {
            return vec![
                vec![Some(0), Some(1), Some(2), Some(3)],
                vec![Some(1), Some(2), Some(3), Some(4)],
                vec![Some(8), Some(7), Some(6), Some(5)],
                vec![Some(9), Some(8), Some(7), Some(6)],
            ];
        }
    }
}

fn is_pos_in_puzzle(pos: &Coords, puzzle: &PuzzleInput) -> bool {
    if pos.y < 0 || pos.y > (puzzle.len() - 1) as i32 {
        return false;
    } else if pos.x < 0 || pos.x > (puzzle[pos.y as usize].len() - 1) as i32 {
        return false;
    }
    return true;
}

fn find_next_step(
    current_value: u32,
    current_position: &Coords,
    direction_to_check: &Direction,
    puzzle: &PuzzleInput,
    coordinates_found: &mut HashSet<Coords>,
) {
    let pos_to_check = current_position + direction_to_check;
    if is_pos_in_puzzle(&pos_to_check, puzzle) {
        let value_at_pos = puzzle[pos_to_check.y as usize][pos_to_check.x as usize];
        match value_at_pos {
            None => return,
            Some(value_at_pos) => {
                if value_at_pos == 9 && current_value == 8 {
                    coordinates_found.insert(Coords {
                        y: pos_to_check.y,
                        x: pos_to_check.x,
                    });
                } else if value_at_pos == current_value + 1 {
                    for next_direction in [
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                        Direction::Left,
                    ] {
                        find_next_step(
                            value_at_pos,
                            &pos_to_check,
                            &next_direction,
                            puzzle,
                            coordinates_found,
                        );
                    }
                }
            }
        }
    }
}

fn find_next_step_two(
    current_value: u32,
    current_position: &Coords,
    direction_to_check: &Direction,
    puzzle: &PuzzleInput,
    coordinates_found: &mut Vec<Coords>,
) {
    let pos_to_check = current_position + direction_to_check;
    if is_pos_in_puzzle(&pos_to_check, puzzle) {
        let value_at_pos = puzzle[pos_to_check.y as usize][pos_to_check.x as usize];
        match value_at_pos {
            None => return,
            Some(value_at_pos) => {
                if value_at_pos == 9 && current_value == 8 {
                    coordinates_found.push(Coords {
                        y: pos_to_check.y,
                        x: pos_to_check.x,
                    });
                } else if value_at_pos == current_value + 1 {
                    for next_direction in [
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                        Direction::Left,
                    ] {
                        find_next_step_two(
                            value_at_pos,
                            &pos_to_check,
                            &next_direction,
                            puzzle,
                            coordinates_found,
                        );
                    }
                }
            }
        }
    }
}

fn solution_one(puzzle: &PuzzleInput) -> usize {
    let mut total = 0;
    for (y, line) in puzzle.iter().enumerate() {
        for (x, digit) in line.iter().enumerate() {
            match digit {
                None => continue,
                Some(digit) => {
                    if *digit == 0 {
                        let current_pos = Coords {
                            y: y as i32,
                            x: x as i32,
                        };
                        let mut coordinates_found: HashSet<Coords> = HashSet::new();
                        for direction in [
                            Direction::Up,
                            Direction::Right,
                            Direction::Down,
                            Direction::Left,
                        ] {
                            find_next_step(
                                *digit,
                                &current_pos,
                                &direction,
                                puzzle,
                                &mut coordinates_found,
                            );
                        }
                        total += coordinates_found.len();
                    }
                }
            }
        }
    }
    total
}

fn solution_two(puzzle: &PuzzleInput) -> usize {
    let mut total = 0;
    for (y, line) in puzzle.iter().enumerate() {
        for (x, digit) in line.iter().enumerate() {
            match digit {
                None => continue,
                Some(digit) => {
                    if *digit == 0 {
                        let current_pos = Coords {
                            y: y as i32,
                            x: x as i32,
                        };
                        let mut coordinates_found: Vec<Coords> = vec![];
                        for direction in [
                            Direction::Up,
                            Direction::Right,
                            Direction::Down,
                            Direction::Left,
                        ] {
                            find_next_step_two(
                                *digit,
                                &current_pos,
                                &direction,
                                puzzle,
                                &mut coordinates_found,
                            );
                        }
                        total += coordinates_found.len();
                    }
                }
            }
        }
    }
    total
}

fn main() {
    let puzzle = read_input();
    println!("Solution One: {}", solution_one(&puzzle));
    println!("Solution Two: {}", solution_two(&puzzle));
}
