use std::{fs, io};
use regex::Regex;

pub fn part1() -> i32 {
    let line = read_file().expect("Unable to read file!");
    let regex = Regex::new(r"(-?\d+)").unwrap();
    let answer: i32 = regex.captures_iter(&line).map(|num| {
        num[1].parse::<i32>().unwrap()
    }).sum();
    answer
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day12.txt")
}