use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

type PuzzleInput = Vec<Vec<char>>;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
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

struct RegionPlot {
    perimeter: usize,
}

fn read_input() -> PuzzleInput {
    let file = File::open("input.txt");
    match file {
        Ok(f) => {
            let buf = BufReader::new(f);
            let grid: PuzzleInput = buf
                .lines()
                .map(|l| l.expect("Could not parse line").chars().collect())
                .collect();
            return grid;
        }
        Err(_) => {
            return vec![
                vec!['A', 'A', 'A', 'A'],
                vec!['B', 'B', 'C', 'D'],
                vec!['B', 'B', 'C', 'C'],
                vec!['E', 'E', 'E', 'C'],
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

fn scan_region(
    region_value: char,
    pos_to_check: &Coords,
    puzzle: &PuzzleInput,
    puzzle_indexes_explored: &mut HashSet<Coords>,
    region_plots_found: &mut HashMap<Coords, RegionPlot>,
) {
    // Check how many neighbours exist in each direction
    puzzle_indexes_explored.insert(*pos_to_check);

    let region_plot = region_plots_found.get_mut(&pos_to_check);
    match region_plot {
        None => {
            region_plots_found.insert(*pos_to_check, RegionPlot { perimeter: 0 });
        }
        Some(_) => {}
    };

    for direction in [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        let new_pos = pos_to_check + &direction;
        if pos_to_check.x == 2 && pos_to_check.y == 2 {
            println!("I'M HERE");
        }
        if is_pos_in_puzzle(&new_pos, puzzle) {
            if puzzle[new_pos.y as usize][new_pos.x as usize] != region_value {
                let region_plot = region_plots_found.get_mut(&pos_to_check);
                match region_plot {
                    None => {
                        region_plots_found.insert(*pos_to_check, RegionPlot { perimeter: 1 });
                    }
                    Some(rp) => {
                        rp.perimeter += 1;
                    }
                };
            } else {
                if !puzzle_indexes_explored.contains(&new_pos) {
                    scan_region(
                        region_value,
                        &new_pos,
                        puzzle,
                        puzzle_indexes_explored,
                        region_plots_found,
                    );
                }
            }
        } else {
            let region_plot = region_plots_found.get_mut(&pos_to_check);
            match region_plot {
                None => {
                    region_plots_found.insert(*pos_to_check, RegionPlot { perimeter: 1 });
                }
                Some(rp) => {
                    rp.perimeter += 1;
                }
            };
        }
    }
}

fn solution_one(puzzle: &PuzzleInput) -> usize {
    let mut solution = 0;
    let mut indexes_explored: HashSet<Coords> = HashSet::new();
    for (y, line) in puzzle.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            let pos = Coords {
                y: y as i32,
                x: x as i32,
            };
            if indexes_explored.contains(&pos) {
                continue;
            }
            // Now to check the spread of the given position
            let mut region_plots_found: HashMap<Coords, RegionPlot> = HashMap::new();
            scan_region(
                *c,
                &pos,
                puzzle,
                &mut indexes_explored,
                &mut region_plots_found,
            );
            let perimeter_total: usize = region_plots_found.values().map(|x| x.perimeter).sum();
            println!(
                "Region: {}, Permiter total: {}, Area total: {}",
                c,
                perimeter_total,
                region_plots_found.values().len()
            );
            if *c == 'O' {
                let plot_coords: Vec<&Coords> = region_plots_found.keys().collect();
                println!("{:?}", plot_coords);
                println!("{:?}", indexes_explored.contains(&Coords { y: 2, x: 2 }));
            }
            solution += perimeter_total * region_plots_found.values().len();
        }
    }
    solution
}

fn main() {
    let puzzle = read_input();
    println!("Solution One: {}", solution_one(&puzzle));
}
