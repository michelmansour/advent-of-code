use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    const NOT_IMPL: &str = "TODO";

    println!("Hello, world!");
    let (d1p1, d1p2) = run(&day1);
    println!("Day 1: p1 {} p2 {}", d1p1, d1p2);

    let (d2p1, d2p2) = run(&day2);
    println!("Day 2: p1 {} p2 {}", d2p1, d2p2);

    let d3p1 = run(&day3);
    println!("Day 3: p1 {} p2 {}", d3p1, NOT_IMPL);
}

fn run<F, T>(func: F) -> T
where
    F: FnOnce() -> Result<T, Box<dyn Error>>,
{
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

fn read_grid(day: i32) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut grid: Vec<Vec<char>> = vec![];
    let lines = read_lines(day)?;

    for line in lines {
        grid.push(line.chars().collect());
    }

    return Ok(grid);
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

fn day2() -> Result<(i32, i32), Box<dyn Error>> {
    let mut sled = 0;
    let mut toboggan = 0;

    for line in read_lines(2)? {
        let v: Vec<&str> = line.split(':').collect();
        let (policy, password) = (v[0].trim(), v[1].trim());

        if check_password_against_sled_policy(policy, password) {
            sled += 1;
        }
        if check_password_against_toboggan_policy(policy, password) {
            toboggan += 1;
        }
    }

    return Ok((sled, toboggan));
}

fn check_password_against_sled_policy(policy: &str, password: &str) -> bool {
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

fn check_password_against_toboggan_policy(policy: &str, password: &str) -> bool {
    let w: Vec<_> = policy.split_ascii_whitespace().collect();
    let (pos_str, character_str) = (w[0].trim(), w[1].trim());
    let posv: Vec<_> = pos_str.split('-').collect();
    let (pos1, pos2) = (posv[0].parse::<usize>(), posv[1].parse::<usize>());

    return xor_char_at_pos(
        character_str.chars().next().unwrap(),
        pos1.unwrap(),
        pos2.unwrap(),
        password,
    );
}

fn xor_char_at_pos(c: char, pos1: usize, pos2: usize, s: &str) -> bool {
    let (p1, p2) = (pos1 - 1, pos2 - 1);
    let cv: Vec<char> = s.chars().collect();
    return (cv[p1] == c && cv[p2] != c) || (cv[p1] != c && cv[p2] == c);
}

fn day3() -> Result<i32, Box<dyn Error>> {
    const TREE: char = '#';
    const MX: usize = 3;
    const MY: usize = 1;

    let grid = read_grid(3)?;
    let bottom = grid.len();
    let right = grid[0].len();

    let mut cur_x = 0;
    let mut cur_y = 0;

    let mut tree_count = 0;

    while cur_y + MY < bottom {
        cur_x += MX;
        cur_y += MY;

        if cur_x >= right {
            cur_x -= right;
        }

        if grid[cur_y][cur_x] == TREE {
            tree_count += 1;
        }
    }

    return Ok(tree_count);
}
