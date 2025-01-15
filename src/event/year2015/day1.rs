use std::{fs, io};

pub fn day1() {
    let input = read_file().expect("Unable to read file!");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
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
    println!("Day 1 - Part 1: {}", floor);
}

fn part2(input: &str) {
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
            println!("Day 1 - Part 2: {}", i + 1);
            break;
        }
    }
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day1.txt")
}