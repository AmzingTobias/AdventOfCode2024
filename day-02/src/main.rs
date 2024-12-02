use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("input.txt").expect("can't open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("can't read");
    let a = f1(s.as_str());
    let b = f2(s.as_str());
    println!("Solution One: {a}");
    println!("Solution Two: {b}");
}


fn f1(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let nums = l
                .split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            is_ok(&nums[..])
        })
        .count()
}

fn is_ok(nums: &[i32]) -> bool {
    let sign = nums[0] < nums[1];
    nums[..]
        .windows(2)
        .all(|w| (w[0] - w[1]).abs() > 0 && (w[0] - w[1]).abs() < 4 && (w[0] < w[1]) == sign)
}

fn f2(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let nums = l
                .split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            is_ok(&nums[..])
                || (0..nums.len()).any(|i| is_ok(&[&nums[..i], &nums[i + 1..]].concat()))
        })
        .count()
}