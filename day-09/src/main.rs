enum Instruction {
    File,
    FreeSpace,
}

impl Instruction {
    fn next(&self) -> Instruction {
        match &self {
            Instruction::File => Instruction::FreeSpace,
            Instruction::FreeSpace => Instruction::File,
        }
    }
}

fn read_input() -> String {
    let input_file = std::fs::read_to_string("input.txt")
        .expect("Input file expected")
        .trim()
        .to_string();
    input_file
}

fn create_puzzle(raw_input: &String) -> Vec<Option<usize>> {
    let mut puzzle: Vec<Option<usize>> = vec![];
    let mut current_instruciton = Instruction::File;
    let mut current_id = 0;
    for i in raw_input.chars() {
        let value_to_use = i.to_digit(10).unwrap() as usize;
        match current_instruciton {
            Instruction::File => {
                for _ in 0..value_to_use {
                    puzzle.push(Some(current_id));
                }
                current_id += 1;
            }
            Instruction::FreeSpace => {
                for _ in 0..value_to_use {
                    puzzle.push(None);
                }
            }
        };
        current_instruciton = current_instruciton.next();
    }
    puzzle
}

#[derive(Debug)]
struct PuzzleTwo {
    space_occupied: usize,
    value: Option<usize>,
}

fn create_puzzle_two(raw_input: &String) -> Vec<PuzzleTwo> {
    let mut puzzle: Vec<PuzzleTwo> = vec![];
    let mut current_instruciton = Instruction::File;
    let mut current_id = 0usize;
    for i in raw_input.chars() {
        let value_to_use = i.to_digit(10).unwrap() as usize;
        puzzle.push(PuzzleTwo {
            space_occupied: value_to_use,
            value: match current_instruciton {
                Instruction::File => {
                    let value = Some(current_id);
                    current_id += 1;
                    value
                }
                Instruction::FreeSpace => None,
            },
        });
        current_instruciton = current_instruciton.next();
    }
    puzzle
}

fn is_puzzle_solved(puzzle: &Vec<Option<usize>>) -> bool {
    let mut free_space_detected = false;
    for c in puzzle {
        match c {
            None => free_space_detected = true,
            Some(_) => {
                if free_space_detected {
                    return false;
                }
            }
        }
    }
    true
}

fn calculate_checksum(puzzle: &Vec<Option<usize>>) -> usize {
    let mut checksum = 0;
    for (index, c) in puzzle.iter().enumerate() {
        match c {
            None => continue,
            Some(d) => checksum += d * index,
        }
    }
    checksum
}

fn calculate_checksum_two(puzzle: &Vec<PuzzleTwo>) -> usize {
    let mut checksum = 0;
    let mut current_index = 0;
    for p in puzzle {
        match p.value {
            None => current_index += 1 * p.space_occupied,
            Some(d) => {
                for _ in 0..(p.space_occupied) {
                    checksum += current_index * d;
                    current_index += 1;
                }
            }
        }
    }
    checksum
}

fn solution_one(mut puzzle: Vec<Option<usize>>) -> usize {
    while !is_puzzle_solved(&puzzle) {
        let first_index_of_free_space = puzzle.iter().position(|&d| d == None);
        match first_index_of_free_space {
            None => break,
            Some(index) => {
                for rev_index in (0..puzzle.len()).rev() {
                    match puzzle[rev_index] {
                        None => continue,
                        Some(d) => {
                            puzzle[index] = Some(d);
                            puzzle[rev_index] = None;
                        }
                    }
                    break;
                }
            }
        }
    }
    calculate_checksum(&puzzle)
}

fn solution_two(mut puzzle: Vec<PuzzleTwo>) -> usize {
    let mut file_ids_moved: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();

    for rev_index in (0..puzzle.len()).rev() {
        if let Some(d) = puzzle[rev_index].value {
            if file_ids_moved.contains_key(&d) {
                continue;
            }

            // Attempt to move the file closer to the beginning
            for index in 0..rev_index {
                if puzzle[index].value.is_none() {
                    if puzzle[index].space_occupied == puzzle[rev_index].space_occupied {
                        // Move the file to this space
                        puzzle[index].value = Some(d);
                        puzzle[rev_index].value = None; // Clear the old spot
                        file_ids_moved.insert(d, 0);
                        break;
                    } else if puzzle[index].space_occupied > puzzle[rev_index].space_occupied {
                        // Split the space and move the file
                        puzzle[index].value = Some(d);
                        let remaining_space =
                            puzzle[index].space_occupied - puzzle[rev_index].space_occupied;
                        puzzle[index].space_occupied = puzzle[rev_index].space_occupied;

                        puzzle[rev_index].value = None; // Clear the old spot

                        // Insert a new empty space for the remainder
                        puzzle.insert(
                            index + 1,
                            PuzzleTwo {
                                space_occupied: remaining_space,
                                value: None,
                            },
                        );

                        file_ids_moved.insert(d, 0);
                        break;
                    }
                }
            }
        }
    }

    calculate_checksum_two(&puzzle)
}

fn main() {
    let raw_input = read_input();
    let puzzle = create_puzzle(&raw_input);
    println!("Solution One: {}", solution_one(puzzle));
    let puzzle = create_puzzle_two(&raw_input);
    println!("Solution Two: {:?}", solution_two(puzzle));
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_puzzle() {
        let result = super::create_puzzle(&"12345".to_string());
        assert_eq!(
            result,
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ]
        );
    }

    #[test]
    fn not_solved() {
        let puzzle_input = vec![
            Some(0),
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
        ];
        let result = super::is_puzzle_solved(&puzzle_input);
        assert_eq!(result, false);
    }

    #[test]
    fn is_solved() {
        let puzzle_input = vec![
            Some(0),
            Some(1),
            Some(1),
            Some(1),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let result = super::is_puzzle_solved(&puzzle_input);
        assert_eq!(result, true);
    }
}
