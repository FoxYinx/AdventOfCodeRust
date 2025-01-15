use std::{fs, io};

pub fn part1() -> i32 {
    let input = read_file().expect("Unable to read file!");
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                eprintln!("Unknown character found");
            }
        }
    }
    floor
}

pub fn part2() -> i32 {
    let input = read_file().expect("Unable to read file!");
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c { 
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                eprintln!("Unknown character found");
            }
        }
        if floor == -1 {
            return (i + 1) as i32;
        }
    }
    -1
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day1.txt")
}