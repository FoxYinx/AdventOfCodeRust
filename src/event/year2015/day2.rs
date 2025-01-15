use std::{fs, io};
use std::cmp::{max, min};
use regex::Regex;

pub fn part1() -> i32 {
    let values = get_values();
    
    let mut total = 0;
    for (v1, v2, v3) in values {
        total += 2 * (v1 * v2 + v1 * v3 + v2 * v3);
        total += min(v1 * v2, min(v1 * v3, v2 * v3));
    }
    total
}

pub fn part2() -> i32 {
    let values = get_values();
    
    let mut total = 0;
    for (v1, v2, v3) in values {
        total += v1 * v2 * v3;
        total += 2 * (v1 + v2 + v3 - max(v1, max(v2, v3)));
    }
    total
}

fn get_values() -> Vec<(i32, i32, i32)> {
    let input = read_file().expect("Unable to read file!");
    let regex = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    regex.captures_iter(&input).map(|caps| {
        let (_, [v1, v2, v3]) = caps.extract();
        (v1.parse::<i32>().unwrap(), v2.parse::<i32>().unwrap(), v3.parse::<i32>().unwrap())
    }).collect()
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day2.txt")
}