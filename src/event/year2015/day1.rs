use std::{fs, io};

pub fn day1() {
    println!("Day 1");
    part1();
}

pub fn part1() {
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
    println!("Part 1: {}", floor);
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day1/part1.txt")
}