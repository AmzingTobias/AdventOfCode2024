use grid::*;
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
            solution += perimeter_total * region_plots_found.values().len();
        }
    }
    solution
}

fn main() {
    let puzzle = read_input();
    println!("Solution One: {}", solution_one(&puzzle));
    println!("Solution Two: {}", solution_two());
}

fn solution_two() -> u32 {
    let mut input = include_str!("../input.txt").lines().peekable();
    let width = input.peek().unwrap().len();
    let mut grid = Grid::from_vec(input.flat_map(|l| l.chars()).collect(), width);

    let mut sum = 0;

    let mut cells = HashSet::new();
    let mut edges: Vec<EdgeDir> = Vec::new();
    for i in 0..width * grid.rows() {
        let x = (i % width) as i16;
        let y = (i / width) as i16;

        let cell = grid[(y as usize, x as usize)];

        if cell == '#' {
            continue;
        }

        cells.clear();
        edges.clear();
        explore(x, y, cell, &mut grid, &mut cells, &mut edges);

        let mut sides: Vec<Vec<EdgeDir>> = Vec::new();
        for edge in edges.iter() {
            // Check for adjacent sides
            let (left, right) = match edge {
                EdgeDir::Horizontal(ed) => (
                    find_edge_in_side(&sides, ed.0, ed.1, ed.2, ed.3, 0, -1),
                    find_edge_in_side(&sides, ed.0, ed.1, ed.2, ed.3, 0, 1),
                ),
                EdgeDir::Vertical(ed) => (
                    find_edge_in_side(&sides, ed.0, ed.1, ed.2, ed.3, -1, 0),
                    find_edge_in_side(&sides, ed.0, ed.1, ed.2, ed.3, 1, 0),
                ),
            };

            if let (Some(left), Some(right)) = (left, right) {
                // Two adjacent sides discovered, link them up
                let mut combined = sides[left].clone();
                combined.extend(sides[right].clone());
                combined.push(*edge);
                sides[left] = combined;
                sides.remove(right);
            } else if let Some(left) = left {
                // Join with left side
                sides[left].push(*edge);
            } else if let Some(right) = right {
                // Join with right side
                sides[right].push(*edge);
            } else {
                // New side discovered
                sides.push(vec![*edge]);
            }
        }

        sum += cells.len() as u32 * sides.len() as u32;
    }
    sum
}

type Edge = (i16, i16, i16, i16);
#[derive(Copy, Clone, PartialEq, Debug)]
enum EdgeDir {
    Horizontal(Edge),
    Vertical(Edge),
}

impl EdgeDir {
    fn unwrap(self) -> Edge {
        match self {
            EdgeDir::Horizontal(edge) | EdgeDir::Vertical(edge) => edge,
        }
    }
}

fn explore(
    x: i16,
    y: i16,
    cell: char,
    grid: &mut Grid<char>,
    cells: &mut HashSet<(i16, i16)>,
    edges: &mut Vec<EdgeDir>,
) -> bool {
    if cells.contains(&(y, x)) {
        return false; // Not an edge
    }

    if let Some(other) = grid.get(y, x) {
        if *other == cell {
            // Same plant does not add perimeter, but does add area (cells)
            cells.insert((y, x));
            grid[(y as usize, x as usize)] = '#';

            if explore(x, y - 1, cell, grid, cells, edges) {
                edges.push(EdgeDir::Vertical((x, y, x, y - 1)));
            }

            if explore(x + 1, y, cell, grid, cells, edges) {
                edges.push(EdgeDir::Horizontal((x, y, x + 1, y)));
            }

            if explore(x, y + 1, cell, grid, cells, edges) {
                edges.push(EdgeDir::Vertical((x, y, x, y + 1)));
            }

            if explore(x - 1, y, cell, grid, cells, edges) {
                edges.push(EdgeDir::Horizontal((x, y, x - 1, y)));
            }

            return false; // Not an edge
        }
    }

    // Is an edge
    true
}

fn find_edge_in_side(
    sides: &Vec<Vec<EdgeDir>>,
    x: i16,
    y: i16,
    tx: i16,
    ty: i16,
    xd: i16,
    yd: i16,
) -> Option<usize> {
    for (index, edges) in sides.iter().enumerate() {
        if edges.iter().any(|edge| {
            let (x1, y1, x2, y2) = edge.unwrap();

            x1 == x + xd && y1 == y + yd && x2 == tx + xd && y2 == ty + yd
        }) {
            return Some(index);
        }
    }

    None
}
