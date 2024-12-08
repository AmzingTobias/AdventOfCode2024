use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
struct Coords {
    x: i32,
    y: i32,
}

impl std::ops::Add<&Coords> for &Coords {
    type Output = Coords;

    fn add(self, coord_to_add: &Coords) -> Coords {
        Coords {
            x: self.x + coord_to_add.x,
            y: self.y + coord_to_add.y,
        }
    }
}

impl std::ops::Sub<&Coords> for &Coords {
    type Output = Coords;

    fn sub(self, coord_to_add: &Coords) -> Coords {
        Coords {
            x: self.x - coord_to_add.x,
            y: self.y - coord_to_add.y,
        }
    }
}

type PuzzleInput = Vec<Vec<char>>;

fn grid_from_file() -> PuzzleInput {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let grid: PuzzleInput = buf
        .lines()
        .map(|l| l.expect("Could not parse line").chars().collect())
        .collect();
    grid
}

fn is_pos_in_grid(pos_to_check: &Coords, grid: &PuzzleInput) -> bool {
    return !((pos_to_check.y < 0)
        || (pos_to_check.y as usize >= grid.len())
        || (pos_to_check.x < 0)
        || (pos_to_check.x as usize >= grid[pos_to_check.y as usize].len()));
}

fn find_points_on_line(
    grid_to_search: &PuzzleInput,
    ref_point_one: &Coords,
    ref_point_two: &Coords,
) -> Vec<Coords> {
    let mut points_found: Vec<Coords> = vec![];
    for (y, line) in grid_to_search.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let dxc = x as i32 - ref_point_one.x;
            let dyc = y as i32 - ref_point_one.y;

            let dxl = ref_point_two.x - ref_point_one.x;
            let dyl = ref_point_two.y - ref_point_one.y;

            let cross = dxc * dyl - dyc * dxl;
            if cross == 0 {
                points_found.push(Coords {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    points_found
}

fn find_antenna_in_line_two(
    mut puzzle_solution: PuzzleInput,
    grid_to_search: &PuzzleInput,
    antenna_found: char,
    pos_antenna_found: Coords,
) -> PuzzleInput {
    for (y, line) in grid_to_search.iter().enumerate() {
        if y < pos_antenna_found.y as usize {
            continue;
        }
        for (x, c) in line.iter().enumerate() {
            if (y == pos_antenna_found.y as usize) && (x < pos_antenna_found.x as usize) {
                continue;
            }
            if (*c == antenna_found)
                && (pos_antenna_found.y != y as i32)
                && (pos_antenna_found.x != x as i32)
            {
                let points_on_line = find_points_on_line(
                    grid_to_search,
                    &pos_antenna_found,
                    &Coords {
                        x: x as i32,
                        y: y as i32,
                    },
                );
                for point in points_on_line {
                    puzzle_solution[point.y as usize][point.x as usize] = '#';
                }
            }
        }
    }
    puzzle_solution
}

fn find_antenna_in_line(
    mut puzzle_solution: PuzzleInput,
    grid_to_search: &PuzzleInput,
    antenna_found: char,
    pos_antenna_found: Coords,
) -> PuzzleInput {
    for (y, line) in grid_to_search.iter().enumerate() {
        if y < pos_antenna_found.y as usize {
            continue;
        }
        for (x, c) in line.iter().enumerate() {
            if (y == pos_antenna_found.y as usize) && (x < pos_antenna_found.x as usize) {
                continue;
            }
            if (*c == antenna_found)
                && (pos_antenna_found.y != y as i32)
                && (pos_antenna_found.x != x as i32)
            {
                let distance_between_two_y: i32 = pos_antenna_found.y.abs_diff(y as i32) as i32;
                let distance_between_two_x: i32 = pos_antenna_found.x.abs_diff(x as i32) as i32;
                if x < pos_antenna_found.x as usize {
                    let coord_to_check = Coords {
                        x: pos_antenna_found.x + distance_between_two_x,
                        y: pos_antenna_found.y - distance_between_two_y,
                    };
                    if is_pos_in_grid(&coord_to_check, &grid_to_search) {
                        puzzle_solution[coord_to_check.y as usize][coord_to_check.x as usize] = '#';
                    }

                    let coord_to_check = Coords {
                        x: x as i32 - distance_between_two_x,
                        y: y as i32 + distance_between_two_y,
                    };
                    if is_pos_in_grid(&coord_to_check, &grid_to_search) {
                        puzzle_solution[coord_to_check.y as usize][coord_to_check.x as usize] = '#';
                    }
                } else {
                    let coord_to_check = Coords {
                        x: pos_antenna_found.x - distance_between_two_x,
                        y: pos_antenna_found.y - distance_between_two_y,
                    };
                    if is_pos_in_grid(&coord_to_check, &grid_to_search) {
                        puzzle_solution[coord_to_check.y as usize][coord_to_check.x as usize] = '#';
                    }

                    let coord_to_check = Coords {
                        x: x as i32 + distance_between_two_x,
                        y: y as i32 + distance_between_two_y,
                    };
                    if is_pos_in_grid(&coord_to_check, &grid_to_search) {
                        puzzle_solution[coord_to_check.y as usize][coord_to_check.x as usize] = '#';
                    }
                }
            }
        }
    }
    puzzle_solution
}

fn solution_one(grid: &PuzzleInput) -> u32 {
    let mut puzzle_solution: PuzzleInput = grid.clone();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '.' {
                puzzle_solution = find_antenna_in_line(
                    puzzle_solution,
                    grid,
                    *c,
                    Coords {
                        x: x as i32,
                        y: y as i32,
                    },
                );
            }
        }
    }
    let mut solution = 0u32;
    for line in puzzle_solution {
        for c in line {
            if c == '#' {
                solution += 1;
            }
        }
    }
    solution
}

fn solution_two(grid: &PuzzleInput) -> u32 {
    let mut puzzle_solution: PuzzleInput = grid.clone();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '.' {
                puzzle_solution = find_antenna_in_line_two(
                    puzzle_solution,
                    grid,
                    *c,
                    Coords {
                        x: x as i32,
                        y: y as i32,
                    },
                );
            }
        }
    }
    let mut solution = 0u32;
    for line in puzzle_solution {
        for c in line {
            if c == '#' {
                solution += 1;
            }
        }
    }
    solution
}

fn main() {
    let grid = grid_from_file();
    println!("Solution One: {}", solution_one(&grid));
    println!("Solution Two: {}", solution_two(&grid));
}
