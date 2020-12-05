use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() {
    println!("Hello, world!");
    println!("Day 1: {}", day1().unwrap());
}

fn read_lines(day: i32) -> Result<Vec<String>, Error> {
    let br = BufReader::new(File::open(format!("./input/day{}.txt", day))?);
    return br.lines().collect();
}

fn day1() -> Result<i32, Error> {
    let nums = match read_lines(1)?
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect() {
            Ok(nums) => nums,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
        };
    let (x, y) = two_sum(nums, 2020);
    return Ok(x * y);
}

fn two_sum(nums: Vec<i32>, target: i32) -> (i32, i32) {
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
