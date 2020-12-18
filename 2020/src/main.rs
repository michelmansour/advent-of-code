use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use lazy_static::lazy_static;
use regex::Regex;

const NOT_IMPL: i64 = -1;

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

    let (d6p1, d6p2) = run(&day6);
    println!("Day 6: p1 {} p2 {}", d6p1, d6p2);

    let (d7p1, d7p2) = run(&day7);
    println!("Day 7: p1 {} p2 {}", d7p1, d7p2);

    let (d8p1, d8p2) = run(&day8);
    println!("Day 8: p1 {} p2 {}", d8p1, d8p2);

    let (d9p1, d9p2) = run(&day9);
    println!("Day 9: p1 {} p2 {}", d9p1, d9p2);

    let (d10p1, d10p2) = run(&day10);
    println!("Day 10: p1 {} p2 {}", d10p1, d10p2);
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

    let mut buffer = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            buffer.push(line);
        }
    }
    Ok(buffer)
}

fn read_grid(day: i32) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut grid = vec![];
    let lines = read_lines(day)?;

    for line in lines {
        grid.push(line.chars().collect());
    }

    Ok(grid)
}

fn read_entries(day: i32, separator: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
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
                for field in line.split_ascii_whitespace() {
                    let kv: Vec<_> = field.split(':').collect();
                    cur_entry.insert(String::from(kv[0].trim()), String::from(kv[1].trim()));
                }
            }
        }
    }

    if cur_entry.len() > 0 {
        entries.push(cur_entry);
    }

    Ok(entries)
}

fn day1() -> Result<(i64, i64), Box<dyn Error>> {
    let nums: Vec<i64> = read_lines(1)?.iter().map(|s| s.parse().unwrap()).collect();

    let (x, y) = two_sum(&nums, 2020);
    let (n, p, q) = three_sum(&nums, 2020);
    Ok((x * y, n * p * q))
}

fn two_sum(nums: &[i64], target: i64) -> (i64, i64) {
    let mut complements = HashSet::new();

    for n in nums {
        let comp = target - n;
        if complements.contains(n) && *n != comp {
            return (*n, comp);
        } else {
            complements.insert(comp);
        }
    }
    (-1, -1)
}

fn three_sum(nums: &[i64], target: i64) -> (i64, i64, i64) {
    for n in nums {
        for p in nums {
            for q in nums {
                if n + p + q == target {
                    return (*n, *p, *q);
                }
            }
        }
    }
    (-1, -1, -1)
}

fn day2() -> Result<(i32, i32), Box<dyn Error>> {
    let mut sled = 0;
    let mut toboggan = 0;

    for line in read_lines(2)? {
        let v: Vec<_> = line.split(':').collect();
        let (policy, password) = (v[0].trim(), v[1].trim());

        if check_password_against_sled_policy(policy, password) {
            sled += 1;
        }
        if check_password_against_toboggan_policy(policy, password) {
            toboggan += 1;
        }
    }

    Ok((sled, toboggan))
}

fn check_password_against_sled_policy(policy: &str, password: &str) -> bool {
    let w: Vec<_> = policy.split_ascii_whitespace().collect();
    let (range_str, character_str) = (w[0].trim(), w[1].trim());
    let range: Vec<_> = range_str.split('-').collect();
    let (min, max) = (
        range[0].parse::<usize>().unwrap(),
        range[1].parse::<usize>().unwrap(),
    );

    let count = count_char_in_str(character_str.chars().next().unwrap(), password);
    count >= min && count <= max
}

fn count_char_in_str(c: char, s: &str) -> usize {
    s.chars().filter(|ch| *ch == c).count()
}

fn check_password_against_toboggan_policy(policy: &str, password: &str) -> bool {
    let w: Vec<_> = policy.split_ascii_whitespace().collect();
    let (pos_str, character_str) = (w[0].trim(), w[1].trim());
    let posv: Vec<_> = pos_str.split('-').collect();
    let (pos1, pos2) = (posv[0].parse(), posv[1].parse());

    xor_char_at_pos(
        character_str.chars().next().unwrap(),
        pos1.unwrap(),
        pos2.unwrap(),
        password,
    )
}

fn xor_char_at_pos(c: char, pos1: usize, pos2: usize, s: &str) -> bool {
    let (p1, p2) = (pos1 - 1, pos2 - 1);
    let cv: Vec<_> = s.chars().collect();
    (cv[p1] == c && cv[p2] != c) || (cv[p1] != c && cv[p2] == c)
}

fn day3() -> Result<(u64, u64), Box<dyn Error>> {
    let grid = read_grid(3)?;

    let part1 = check_slope(&grid, 3, 1);

    let mut total_trees_product = 1u64;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for &(x, y) in &slopes {
        total_trees_product *= check_slope(&grid, x, y);
    }

    Ok((part1, total_trees_product))
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

    tree_count
}

fn day4() -> Result<(i32, i32), Box<dyn Error>> {
    const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_naive_entries = 0;
    let mut valid_entries = 0;

    let passport_entries = read_entries(4, "")?;
    for entry in &passport_entries {
        let mut is_valid_props = true;
        let mut is_valid_values = true;
        for &field in &REQUIRED_FIELDS {
            if let Some(value) = entry.get(field) {
                if is_valid_values {
                    is_valid_values = match field {
                        "byr" => check_birth_year(value),
                        "iyr" => check_issue_year(value),
                        "eyr" => check_expiration_year(value),
                        "hgt" => check_height(value),
                        "hcl" => check_hair_color(value),
                        "ecl" => check_eye_color(value),
                        "pid" => check_passport_id(value),
                        _ => true,
                    };
                }
            } else {
                is_valid_props = false;
                is_valid_values = false;
                break;
            }
        }
        if is_valid_props {
            valid_naive_entries += 1
        }
        if is_valid_values {
            valid_entries += 1;
        }
    }

    return Ok((valid_naive_entries, valid_entries));
}

fn check_num_in_range(start: i32, end: i32, value: &str) -> bool {
    match value.parse::<i32>() {
        Ok(value) => value >= start && value <= end,
        Err(_) => false,
    }
}

fn check_birth_year(birth_year: &str) -> bool {
    check_num_in_range(1920, 2002, birth_year)
}

fn check_issue_year(issue_year: &str) -> bool {
    check_num_in_range(2010, 2020, issue_year)
}

fn check_expiration_year(expiration_year: &str) -> bool {
    check_num_in_range(2020, 2030, expiration_year)
}

fn check_height(height: &str) -> bool {
    let unit_idx = height.len() - 2;
    let valid_hgt_range = match &height[unit_idx..] {
        "cm" => (150, 193),
        "in" => (59, 76),
        _ => return false,
    };

    check_num_in_range(valid_hgt_range.0, valid_hgt_range.1, &height[..unit_idx])
}

fn check_eye_color(eye_color: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color)
}

fn check_hair_color(hair_color: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(hair_color)
}

fn check_passport_id(passport_id: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    RE.is_match(passport_id)
}

fn day5() -> Result<(i32, i32), Box<dyn Error>> {
    let get_seat_id = |(r, c): &(i32, i32)| r * 8 + c;

    let seats: Vec<_> = read_lines(5)?
        .iter()
        .map(|p| check_boarding_pass(p, 128, 8))
        .collect();

    let max_seat = match seats.iter().map(get_seat_id).max() {
        Some(x) => x,
        None => -1,
    };

    let missing_seat = find_missing_seat(seats, &get_seat_id);

    Ok((max_seat, missing_seat))
}

fn check_boarding_pass(boarding_pass: &str, num_rows: i32, num_cols: i32) -> (i32, i32) {
    let row_part_len = (num_cols - 1) as usize;
    (
        find_seat_row(&boarding_pass[0..row_part_len], num_rows),
        find_seat_col(&boarding_pass[row_part_len..], num_cols),
    )
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

    mid
}

fn find_seat_row(row_part: &str, num_rows: i32) -> i32 {
    seat_binary_search(row_part, 'F', num_rows)
}

fn find_seat_col(col_part: &str, num_cols: i32) -> i32 {
    seat_binary_search(col_part, 'L', num_cols)
}

fn find_missing_seat<F>(seats: Vec<(i32, i32)>, id_func: F) -> i32
where
    F: Fn(&(i32, i32)) -> i32,
{
    let mut seat_ids: Vec<_> = seats.iter().map(&id_func).collect();
    seat_ids.sort();

    let mut expected_cur_seat_id = seat_ids[0];
    for seat_id in seat_ids {
        if expected_cur_seat_id == seat_id {
            expected_cur_seat_id += 1;
        } else {
            return expected_cur_seat_id;
        }
    }
    expected_cur_seat_id + 1
}

fn day6() -> Result<(i32, i32), Box<dyn Error>> {
    let lines = read_lines(6)?;
    let mut group_answers = Vec::new();

    let mut cur_group = HashMap::new();
    let mut group_size = 0;
    for line in lines {
        if line.is_empty() {
            group_answers.push((group_size, cur_group));
            cur_group = HashMap::new();
            group_size = 0;
        } else {
            group_size += 1;
            for c in line.chars() {
                let counter = cur_group.entry(c).or_insert(0);
                *counter += 1;
            }
        }
    }
    if !cur_group.is_empty() {
        group_answers.push((group_size, cur_group));
    }

    let total_uniq_qs_per_group: usize = group_answers.iter().map(|(_, m)| m.len()).sum();
    let total_univ_qs_per_group: usize = group_answers
        .iter()
        .map(|(size, counts)| {
            counts
                .iter()
                .filter(|&(_, v)| v == size)
                .map(|(k, _)| k)
                .count()
        })
        .sum();

    Ok((
        total_uniq_qs_per_group as i32,
        total_univ_qs_per_group as i32,
    ))
}

fn day7() -> Result<(i32, i32), Box<dyn Error>> {
    let lines = read_lines(7)?;

    let mut contains: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut contained_by: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let bags: Vec<&str> = line.trim_end_matches('.').split(" bags contain ").collect();
        if bags[1] != "no other bags" {
            let containing_bag = bags[0];
            let contained_bags = bags[1].split(',').fold(HashMap::new(), |mut acc, s| {
                let bag_props: Vec<&str> = s.trim().split_ascii_whitespace().collect();
                let mut bag_color = String::from(bag_props[1]);
                bag_color.push_str(" ");
                bag_color.push_str(bag_props[2]);
                acc.insert(bag_color, bag_props[0].parse::<i32>().unwrap());
                acc
            });

            for bag in contained_bags.keys() {
                let v = contained_by.entry(String::from(bag)).or_insert(Vec::new());
                v.push(String::from(containing_bag));
            }
            contains.insert(String::from(containing_bag), contained_bags);
        }
    }
    return Ok((
        count_reachable_nodes("shiny gold", &contained_by) as i32,
        count_inner_bags("shiny gold", &contains),
    ));
}

fn count_reachable_nodes(start: &str, graph: &HashMap<String, Vec<String>>) -> usize {
    let mut stack = vec![start];
    let mut outermost_colors = HashSet::new();

    let empty_neighbors = Vec::new();
    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        let neighbors = match graph.get(top) {
            Some(neighbors) => neighbors,
            None => &empty_neighbors,
        };
        for n in neighbors {
            stack.push(n);
            outermost_colors.insert(n);
        }
    }
    outermost_colors.len()
}

fn count_inner_bags(start: &str, graph: &HashMap<String, HashMap<String, i32>>) -> i32 {
    // println!("{:#?}", graph);
    match graph.get(start) {
        Some(nested) => {
            let mut total = 0;
            for b in nested.keys() {
                let c = nested.get(b).unwrap();
                total += c + c * count_inner_bags(b, graph);
            }
            total
        }
        None => 0,
    }
}

fn day8() -> Result<(i32, i32), Box<dyn Error>> {
    let lines = read_lines(8)?;
    let program: Vec<(&str, i32)> = lines
        .iter()
        .map(|l| {
            let inst: Vec<&str> = l.split_ascii_whitespace().collect();
            (inst[0], inst[1].parse().unwrap())
        })
        .collect();

    Ok((find_infinite_loop(&program).0, fix_program(&program)))
}

fn find_infinite_loop(program: &[(&str, i32)]) -> (i32, bool) {
    let program_length = program.len() as i32;

    let mut i = 0;
    let mut found_loop = false;
    let mut visited = HashSet::new();
    let mut accumulator = 0;

    while !found_loop && i < program_length && i >= 0 {
        if visited.contains(&i) {
            found_loop = true
        } else {
            visited.insert(i);
            let (inst, n) = &program[i as usize];
            match inst.as_ref() {
                "acc" => {
                    i += 1;
                    accumulator += n;
                }
                "jmp" => i += n,
                _ => i += 1, // nop
            }
        }
    }

    (accumulator, found_loop)
}

fn fix_program(program: &[(&str, i32)]) -> i32 {
    let mut broken = true;
    let mut line = 0;
    let mut acc = 0;

    let mut prgm = Vec::new();
    for line in program {
        prgm.push((line.0, line.1));
    }

    while broken && line < program.len() {
        let mut run = false;
        let (orig_inst, value) = program[line];
        let new_inst = match orig_inst {
            "jmp" => {
                run = true;
                "nop"
            }
            "nop" => {
                run = true;
                "jmp"
            }
            _ => orig_inst,
        };
        if run {
            prgm[line] = (new_inst, value);
            let result = find_infinite_loop(&prgm);
            acc = result.0;
            broken = result.1;
            prgm[line] = (orig_inst, value);
        }
        line += 1;
    }
    acc
}

fn day9() -> Result<(i64, i64), Box<dyn Error>> {
    let lines: Vec<i64> = read_lines(9)?.iter().map(|l| l.parse().unwrap()).collect();

    Ok((
        find_first_invalid_xmas(&lines, 25),
        find_weakness_xmas(&lines, 25),
    ))
}

fn find_first_invalid_xmas(data: &Vec<i64>, window_length: usize) -> i64 {
    let mut start = 0;
    let mut window = &data[start..window_length];
    let mut result = 0;

    for i in window_length..data.len() {
        if !validate_next_xmas(window, data[i]) {
            result = data[i];
            break;
        }
        start += 1;
        window = &data[start..start + window_length];
    }
    result
}

fn validate_next_xmas(window: &[i64], target: i64) -> bool {
    two_sum(window, target) != (-1, -1)
}

fn find_weakness_xmas(data: &Vec<i64>, window_length: usize) -> i64 {
    let invalid = find_first_invalid_xmas(&data, window_length);
    let mut result = 0;

    for i in 0..data.len() {
        let start = i;
        let mut j = i;
        let mut sum = 0;
        while sum < invalid {
            sum += data[j];
            j += 1;
        }
        if sum == invalid {
            let block = &data[start..j];
            let max = block.iter().max().unwrap();
            let min = block.iter().min().unwrap();
            result = min + max;
            break;
        }
    }
    result
}

fn day10() -> Result<(i32, i32), Box<dyn Error>> {
    let data: Vec<i32> = read_lines(10)?.iter().map(|d| d.parse().unwrap()).collect();

    let joltage_diffs = calc_joltage_diffs(&data);
    Ok((joltage_diffs.0 * joltage_diffs.2, NOT_IMPL as i32))
}

fn calc_joltage_diffs(joltages: &[i32]) -> (i32, i32, i32) {
    let joltage_chain = make_full_joltage_chain(joltages);

    let mut ones = 0;
    let mut twos = 0;
    let mut threes = 1; // plus the 3 at the end

    let mut cur_joltage = 0;

    for j in joltage_chain {
        match j - cur_joltage {
            1 => ones += 1,
            2 => twos += 1,
            _ => threes += 1,
        };
        cur_joltage = j;
    }

    (ones, twos, threes)
}

fn make_full_joltage_chain(joltages: &[i32]) -> Vec<i32> {
    let mut copy = Vec::new();
    for j in joltages {
        copy.push(*j);
    }
    copy.sort();
    copy
}
