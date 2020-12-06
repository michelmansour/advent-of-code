use std::process;
use std::error::Error;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let (d1p1, d1p2) = run(&day1);
    println!("Day 1: p1 {} p2 {}", d1p1, d1p2);
    println!("Day 2: {}", run(&day2));
}

fn run<F, T>(func: F) -> T where
    F: FnOnce() -> Result<T, Box<dyn Error>> {
    return func().unwrap_or_else(|err| {
        eprintln!("Something bad happened: {}", err);
        process::exit(1);
    });
}

fn read_lines(day: i32) -> Result<Vec<String>, Box<dyn Error>> {
    let f = File::open(format!("./input/day{}.txt", day))?;
    let lines = BufReader::new(f).lines();

    let mut buffer: Vec<String> = vec![];
    for line in lines {
        if let Ok(x) = line {
            buffer.push(x);
        }
    }
    return Ok(buffer);
}

fn day1() -> Result<(i32, i32), Box<dyn Error>> {
    let nums = read_lines(1)?
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let (x, y) = two_sum(&nums, 2020);
    let (n, p, q) = three_sum(&nums, 2020);
    return Ok((x * y, n * p * q));
}

fn two_sum(nums: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut complements = HashSet::new();

    for n in nums.iter() {
        let comp = target - n;
        if complements.contains(n) {
            return (*n, comp);
        } else {
            complements.insert(comp);
        }
    }
    return (-1, 1);
}

fn three_sum(nums: &Vec<i32>, target: i32) -> (i32, i32, i32) {
    for n in nums.iter() {
        for p in nums.iter() {
            for q in nums.iter() {
                if n + p + q == target {
                    return (*n, *p, *q);
                }
            }
        }
    }
    return (-1, -1, -1);
}

fn day2() -> Result<usize, Box<dyn Error>> {
    let num_matched = read_lines(2)?
        .iter()
        .map(|line| {
            let v: Vec<&str> = line.split(':').collect();
            return (v[0].trim(), v[1].trim());
        })
        .filter(|(policy, password)| check_password_against_policy(policy, password))
        .count();

    return Ok(num_matched);
}

fn check_password_against_policy(policy: &str, password: &str) -> bool {
    let w: Vec<_> = policy.split_ascii_whitespace().collect();
    let (range_str, character_str) = (w[0].trim(), w[1].trim());
    let range: Vec<_> = range_str.split('-').collect();
    let (min, max) = (range[0].parse::<i32>(), range[1].parse::<i32>());

    let count = count_char_in_str(character_str.chars().next().unwrap(), password);
    return count >= min.unwrap() && count <= max.unwrap();
}

fn count_char_in_str(c: char, s: &str) -> i32 {
    let mut count = 0;
    for ch in s.chars() {
        if c == ch {
            count += 1;
        }
    }
    return count;
}
