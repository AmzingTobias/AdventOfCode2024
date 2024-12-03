use regex::Regex;
use std::fs;

fn solution_one(input: &str) -> u32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut sum = 0;
    for mat in re.find_iter(input) {
        let re_digit = Regex::new(r"\d+").unwrap();
        let matches: Vec<u32> = re_digit
            .find_iter(mat.as_str())
            .map(|m| m.as_str().parse::<u32>().expect("Should be a digit"))
            .collect();
        let product: u32 = matches.iter().copied().reduce(|a, b| a * b).unwrap();
        sum += product;
    }
    sum
}

fn solution_two(input: &str) -> u32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for mat in re.find_iter(input) {
        let match_as_str = mat.as_str();
        if match_as_str == "do()" {
            enabled = true;
        } else if match_as_str == "don't()" {
            enabled = false;
        } else {
            if enabled {
                let re_digit = Regex::new(r"\d+").unwrap();
                let matches: Vec<u32> = re_digit
                    .find_iter(match_as_str)
                    .map(|m| m.as_str().parse::<u32>().expect("Should be a digit"))
                    .collect();
                let product: u32 = matches.iter().copied().reduce(|a, b| a * b).unwrap();
                sum += product;
            }
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file missing");

    println!("Solution One: {}", solution_one(&input));
    println!("Solution Two: {}", solution_two(&input));
}
