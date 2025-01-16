use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

pub fn part1() -> usize {
    let input = read_file().expect("Unable to read file!");
    let mut total: usize = 0;
    for line in input {
        total += 2;
        total += line.matches(r"\\").count();
        total += line.matches(r#"\""#).count();
        total += line.matches(r"\x").count() * 3;
    }
    
    total
}

pub fn part2() -> usize {
    let input = read_file().expect("Unable to read file!");
    let mut total: usize = 0;
    for line in input {
        total += 2;
        total += line.matches('"').count();
        total += line.matches('\\').count();
    }
    
    total
}

fn read_file() -> io::Result<Vec<String>> {
    BufReader::new(File::open("ressources/year2015/day8.txt")?).lines().collect()
}