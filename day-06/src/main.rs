use std::{fs::File, io::{BufRead, BufReader}, ops::Add};


struct Coords {
    x: i32,
    y: i32
}

impl Add<&Directions> for &Coords {
    type Output = Coords;

    fn add(self, dir: &Directions) -> Coords {
        let coord_to_add= dir.value();
        Coords {
            x: self.x + coord_to_add.x,
            y: self.y + coord_to_add.y,
        }
    }
}
enum Directions {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Directions {
    fn value(&self) -> Coords {
        match self {
            Directions::UP => Coords { x: 0, y: -1 },
            Directions::DOWN => Coords { x: 0, y: 1 },
            Directions::RIGHT => Coords { x: 1, y: 0 },
            Directions::LEFT => Coords { x: -1, y: 0 },
        }
    }
    
    fn turn_right(&self) -> Directions {
        match self {
            Directions::UP => Directions::RIGHT,
            Directions::RIGHT => Directions::DOWN,
            Directions::DOWN => Directions::LEFT,
            Directions::LEFT => Directions::UP
        }
    }
}

fn grid_from_file() -> Vec<Vec<char>> {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let grid: Vec<Vec<char>> = buf
        .lines()
        .map(|l| l.expect("Could not parse line").chars().collect())
        .collect();
    grid
}

fn get_start_pos(grid: &Vec<Vec<char>>) -> Option<Coords> {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|x| *x == '^') {
            return Some(Coords{x: j as i32, y: i as i32});
        }
    }
    None
}

fn traverse_grid(
    grid: &mut Vec<Vec<char>>, 
    mut current_pos: Coords, 
    mut direction_to_move: Directions, 
    loop_limit: u32
) -> Vec<Vec<char>> {
    let mut loop_count = 0u32;

    loop {
        if loop_limit != 0 && loop_count > loop_limit {
            return vec![vec![]];
        }

        loop_count += 1;

        // Calculate the next position
        let next_pos = &current_pos + &direction_to_move;

        if next_pos.y < 0 || next_pos.y as usize >= grid.len() || 
           next_pos.x < 0 || next_pos.x as usize >= grid[next_pos.y as usize].len() {
            // If out of bounds, mark current position as 'X' and stop
            grid[current_pos.y as usize][current_pos.x as usize] = 'X';
            break;
        } else if grid[next_pos.y as usize][next_pos.x as usize] == '#' {
            // If hitting an obstacle, turn right
            direction_to_move = direction_to_move.turn_right();
            continue;
        }

        grid[current_pos.y as usize][current_pos.x as usize] = 'X';

        current_pos = next_pos;
    }

    grid.clone()
}


fn solution_one(mut grid: Vec<Vec<char>>) -> u32 {
    let start_pos = get_start_pos(&grid).expect("");
    let new_grid = traverse_grid(&mut grid, start_pos, Directions::UP, 0);
    let mut count = 0u32;
    for line in new_grid {
        for c in line {
            if c == 'X' {
                count += 1;
            }
        }
    }
    count
}

fn solution_two(grid: Vec<Vec<char>>, solution_one_answer: u32) -> u32 {
    let mut count = 0u32;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' || grid[i][j] == '^' {
                continue;
            }

            let mut test_grid = grid.clone();
            test_grid[i][j] = '#';

            if let Some(start_pos) = get_start_pos(&test_grid) {
                let new_grid = traverse_grid(&mut test_grid, start_pos, Directions::UP, solution_one_answer * 2);

                if new_grid[0].len() == 0 {
                    count += 1;
                }
            }
        }
    }

    count
}


fn main() {
    let grid = grid_from_file();
    let solution_one = solution_one(grid);
    println!("Solution One: {}", solution_one);
    let grid = grid_from_file();
    println!("Solution Two: {}", solution_two(grid, solution_one));
}
