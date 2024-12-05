use std::{
    collections::{HashMap, HashSet}, fs::File, io::{prelude::*, BufReader}
};

fn read_input() -> (Vec<String>, Vec<String>) {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    let original_buffer: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let mut ordering_rules: Vec<String> = vec![];
    let mut update_order: Vec<String> = vec![];
    for line in original_buffer {
        if line.contains("|") {
            ordering_rules.push(line);
        } else if line.contains(","){
            update_order.push(line);
        }
    }
    return (ordering_rules, update_order);
}

fn build_order_map(ordering_rules: Vec<String>) -> HashMap<u32, Vec<u32>>  {
    let mut order_map:HashMap<u32, Vec<u32>> = HashMap::new();
    for line in ordering_rules {
        let mut iter = line.split("|");
        let page_before: u32 = iter.next().unwrap().parse().unwrap();
        let page_after: u32 = iter.next().unwrap().parse().unwrap();
        order_map.entry(page_before)
            .or_insert_with(Vec::new)
            .push(page_after);
    }
    order_map
}

fn solution_one(update_order: Vec<String>, order_map: HashMap<u32, Vec<u32>>) -> u32 {
    let mut solution = 0;
    for line in update_order {
        let update = line.split(",");
        let length = update.clone().count();
        let mut numbers_so_far :Vec<u32> = vec![];
        let mut middle = 0u32;
        'outer: for (index, v) in update.enumerate() {

            let digit: u32 = v.parse().unwrap();
            if index == (length / 2) {
                middle = digit;
            }
            
            let numbers_that_cannot_be_before = order_map.get(&digit);
            match numbers_that_cannot_be_before {
                None => {numbers_so_far.push(digit);},
                Some(n_before) => {
                    let s1: HashSet<_> = numbers_so_far.iter().copied().collect();
                    let s2: HashSet<_> = n_before.iter().copied().collect();
                    let diff = s1.intersection(&s2).count();
                    if diff > 0 {
                        if index >= (length / 2) {
                            solution -= middle;
                        }
                        break 'outer;
                    }
                    numbers_so_far.push(digit);
                }
            }
        }
        solution += middle;
    }
    solution
}

fn main() {
    let (ordering_rules_raw, update_order) = read_input();
    let order_map= build_order_map(ordering_rules_raw);
    println!("Solution One: {}", solution_one(update_order, order_map));
}
