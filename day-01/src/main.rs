use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::HashMap
};

fn lines_from_file() -> (Vec<u32>, Vec<u32>) {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let original_buffer: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let mut first_list: Vec<u32> = vec![];
    let mut second_list: Vec<u32> = vec![];
    for line in original_buffer {
        let mut split = line.split_whitespace();
        first_list.push(split.next().unwrap().parse().unwrap());
        second_list.push(split.next().unwrap().parse().unwrap());
    }
    return (first_list, second_list);
}

fn solution_one() -> u32 {
    let (mut first_list, mut second_list) = lines_from_file();

    first_list.sort();
    second_list.sort();

    let mut total_difference = 0u32;
    for it in first_list.iter().zip(second_list.iter_mut()) {
        let (ai, bi) = it;
        total_difference += (*bi).abs_diff(*ai);
    }
   total_difference
}

fn solution_two() -> usize {
    let (first_list, second_list) = lines_from_file();
    let mut number_of_occurences:HashMap<u32, usize> = HashMap::new();
    for value in first_list {
        if !number_of_occurences.contains_key(&value) {
            let count = second_list.iter().filter(|&n| *n == value).count();
            number_of_occurences.insert(value, count * usize::try_from(value).unwrap());
        }
    }
    number_of_occurences.values().sum()
}

fn main() {
    println!("Solution One: {}", solution_one());
    println!("Solution Two: {}", solution_two());
}