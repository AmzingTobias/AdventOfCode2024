use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn grid_from_file() -> Vec<Vec<char>> {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let grid: Vec<Vec<char>> = buf
        .lines()
        .map(|l| l.expect("Could not parse line").chars().collect())
        .collect();
    grid
}

fn solution_one(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            if *symbol == 'X' {
                count += check_forwards(grid, y, x);
                count += check_backwards(grid, y, x);
                count += check_up(grid, y, x);
                count += check_down(grid, y, x);
                count += check_diagonal(grid, y, x);
            }
        }
    }
    count
}

fn check_backwards(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u32 {
    if x >= 3 {
        if (grid[y][x - 1] == 'M') && (grid[y][x - 2] == 'A') && (grid[y][x - 3] == 'S') {
            return 1;
        }
    }
    0
}

fn check_forwards(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u32 {
    if x < (grid[y].len() - 3) {
        if (grid[y][x + 1] == 'M') && (grid[y][x + 2] == 'A') && (grid[y][x + 3] == 'S') {
            return 1;
        }
    }
    0
}

fn check_up(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u32 {
    if y >= 3 {
        if (grid[y - 1][x] == 'M') && (grid[y - 2][x] == 'A') && (grid[y - 3][x] == 'S') {
            return 1;
        }
    }
    0
}

fn check_down(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u32 {
    if y < (grid.len() - 3) {
        if (grid[y + 1][x] == 'M') && (grid[y + 2][x] == 'A') && (grid[y + 3][x] == 'S') {
            return 1;
        }
    }
    0
}

fn check_diagonal(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u32 {
    let mut found = 0u32;
    if (x >= 3) && (y >= 3) {
        if (grid[y - 1][x - 1] == 'M') && (grid[y - 2][x - 2] == 'A') && (grid[y - 3][x - 3] == 'S')
        {
            found += 1;
        }
    }

    if (x < (grid[y].len() - 3)) && (y >= 3) {
        if (grid[y - 1][x + 1] == 'M') && (grid[y - 2][x + 2] == 'A') && (grid[y - 3][x + 3] == 'S')
        {
            found += 1;
        }
    }

    if (x >= 3) && (y < (grid.len() - 3)) {
        if (grid[y + 1][x - 1] == 'M') && (grid[y + 2][x - 2] == 'A') && (grid[y + 3][x - 3] == 'S')
        {
            found += 1;
        }
    }

    if (x < (grid[y].len() - 3)) && (y < (grid.len() - 3)) {
        if (grid[y + 1][x + 1] == 'M') && (grid[y + 2][x + 2] == 'A') && (grid[y + 3][x + 3] == 'S')
        {
            found += 1;
        }
    }

    found
}

fn solution_two(grid: &Vec<Vec<char>>) -> u32 {
    let mut result = 0u32;
    for (y, line) in grid.iter().enumerate() {
        if y == 0 || y == grid.len() - 1 {
            continue;
        }
        for (x, symbol) in line.iter().enumerate() {
            if x == 0 || x == grid[y].len() - 1 {
                continue;
            }
            if *symbol == 'A' {
                if ((grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M')
                    || (grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S'))
                    && ((grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M')
                        || (grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S'))
                {
                    result += 1;
                }
            }
        }
    }
    result
}

fn main() {
    let grid = grid_from_file();
    println!("Solution One: {}", solution_one(&grid));
    println!("Solution Two: {}", solution_two(&grid));
}
