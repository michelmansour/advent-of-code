use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use lazy_static::lazy_static;
use regex::Regex;

const NOT_IMPL: &str = "TODO";

fn main() {
    println!("Hello, world!");
    let (d1p1, d1p2) = run(&day1);
    println!("Day 1: p1 {} p2 {}", d1p1, d1p2);

    let (d2p1, d2p2) = run(&day2);
    println!("Day 2: p1 {} p2 {}", d2p1, d2p2);

    let (d3p1, d3p2) = run(&day3);
    println!("Day 3: p1 {} p2 {}", d3p1, d3p2);

    let (d4p1, d4p2) = run(&day4);
    println!("Day 4: p1 {} p2 {}", d4p1, d4p2);

    let (d5p1, d5p2) = run(&day5);
    println!("Day 5: p1 {} p2 {}", d5p1, d5p2);
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

fn read_entries(
    day: i32,
    separator: String,
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let f = File::open(format!("./input/day{}.txt", day))?;
    let lines = BufReader::new(f).lines();

    let mut entries = vec![];

    let mut cur_entry = HashMap::new();
    for line in lines {
        if let Ok(line) = line {
            if line == separator {
                entries.push(cur_entry);
                cur_entry = HashMap::new();
            } else {
                let fields = line.split_ascii_whitespace();
                for field in fields {
                    let kv: Vec<&str> = field.split(':').collect();
                    cur_entry.insert(String::from(kv[0].trim()), String::from(kv[1].trim()));
                }
            }
        }
    }

    if cur_entry.len() > 0 {
        entries.push(cur_entry);
    }

    return Ok(entries);
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

fn day3() -> Result<(u64, u64), Box<dyn Error>> {
    let grid = read_grid(3)?;

    let part1 = check_slope(&grid, 3, 1);

    let mut total_trees_product = 1u64;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (x, y) in slopes.iter() {
        total_trees_product *= check_slope(&grid, *x, *y);
    }

    return Ok((part1, total_trees_product));
}

fn check_slope(grid: &Vec<Vec<char>>, mx: usize, my: usize) -> u64 {
    const TREE: char = '#';

    let bottom = grid.len();
    let right = grid[0].len();

    let mut cur_x = 0;
    let mut cur_y = 0;

    let mut tree_count = 0;

    while cur_y + my < bottom {
        cur_x += mx;
        cur_y += my;

        if cur_x >= right {
            cur_x -= right;
        }

        if grid[cur_y][cur_x] == TREE {
            tree_count += 1;
        }
    }

    return tree_count;
}

fn day4() -> Result<(i32, i32), Box<dyn Error>> {
    const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_naive_entries = 0;
    let mut valid_entries = 0;

    let passport_entries = read_entries(4, "".to_string())?;
    for entry in passport_entries {
        let mut is_naive_valid = true;
        let mut is_valid = true;
        for field in REQUIRED_FIELDS {
            if entry.contains_key(*field) {
                let value = entry.get(*field).unwrap(); // we just confirmed the key exists
                if is_valid {
                    is_valid = match field {
                        &"byr" => check_birth_year(value),
                        &"iyr" => check_issue_year(value),
                        &"eyr" => check_expiration_year(value),
                        &"hgt" => check_height(value),
                        &"hcl" => check_hair_color(value),
                        &"ecl" => check_eye_color(value),
                        &"pid" => check_passport_id(value),
                        _ => true,
                    };
                }
            } else {
                is_naive_valid = false;
                is_valid = false;
                break;
            }
        }
        if is_naive_valid {
            valid_naive_entries += 1
        }
        if is_valid {
            valid_entries += 1;
        }
    }

    return Ok((valid_naive_entries, valid_entries));
}

fn check_num_in_range(start: i32, end: i32, value: &str) -> bool {
    match value.parse::<i32>() {
        Ok(value) => return value >= start && value <= end,
        Err(_) => return false,
    }
}

fn check_birth_year(birth_year: &str) -> bool {
    return check_num_in_range(1920, 2002, birth_year);
}

fn check_issue_year(issue_year: &str) -> bool {
    return check_num_in_range(2010, 2020, issue_year);
}

fn check_expiration_year(expiration_year: &str) -> bool {
    return check_num_in_range(2020, 2030, expiration_year);
}

fn check_height(height: &str) -> bool {
    let valid_hgt_range;
    let unit_idx;

    if let Some(idx) = height.find("cm") {
        valid_hgt_range = (150, 193);
        unit_idx = idx;
    } else if let Some(idx) = height.find("in") {
        valid_hgt_range = (59, 76);
        unit_idx = idx;
    } else {
        return false;
    }

    let hgt_val = &height[0..unit_idx];
    return check_num_in_range(valid_hgt_range.0, valid_hgt_range.1, hgt_val);
}

fn check_eye_color(eye_color: &str) -> bool {
    const VALID_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    return VALID_COLORS.contains(&eye_color);
}

fn check_hair_color(hair_color: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    return RE.is_match(hair_color);
}

fn check_passport_id(passport_id: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    return RE.is_match(passport_id);
}

fn day5() -> Result<(i32, i32), Box<dyn Error>> {
    let get_seat_id = |(r, c): &(i32, i32)| r * 8 + c;

    let seats: Vec<(i32, i32)> = read_lines(5)?
        .iter()
        .map(|p| check_boarding_pass(p, 128, 8))
        .collect();

    let max_seat = match seats.iter().map(get_seat_id).max() {
        Some(x) => x,
        None => -1,
    };

    let missing_seat = find_missing_seat(seats, &get_seat_id);

    return Ok((max_seat, missing_seat));
}

fn check_boarding_pass(boarding_pass: &str, num_rows: i32, num_cols: i32) -> (i32, i32) {
    let row_part_len = (num_cols - 1) as usize;
    return (
        find_seat_row(&boarding_pass[0..row_part_len], num_rows),
        find_seat_col(&boarding_pass[row_part_len..], num_cols),
    );
}

fn seat_binary_search(directions: &str, lower_half_ind: char, size: i32) -> i32 {
    let (mut first, mut last, mut mid) = (0, size - 1, size / 2 - 1);

    for dir in directions.chars() {
        if dir == lower_half_ind {
            last = mid;
        } else {
            first = mid + 1;
        }
        mid = (last - first) / 2 + first;
    }

    return mid;
}

fn find_seat_row(row_part: &str, num_rows: i32) -> i32 {
    return seat_binary_search(row_part, 'F', num_rows);
}

fn find_seat_col(col_part: &str, num_cols: i32) -> i32 {
    return seat_binary_search(col_part, 'L', num_cols);
}

fn find_missing_seat<F>(seats: Vec<(i32, i32)>, id_func: F) -> i32
where
    F: Fn(&(i32, i32)) -> i32,
{
    let mut seat_ids: Vec<i32> = seats.iter().map(&id_func).collect();
    seat_ids.sort();

    let mut expected_cur_seat_id = seat_ids[0];
    for seat_id in seat_ids {
        if expected_cur_seat_id == seat_id {
            expected_cur_seat_id += 1;
        } else {
            return expected_cur_seat_id;
        }
    }
    return expected_cur_seat_id + 1;
}
