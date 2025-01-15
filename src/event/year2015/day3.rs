use std::{fs, io};
use std::collections::HashSet;

pub fn day3() {
    println!("Day 3");
    let input = read_file().expect("Unable to read file!");
    part1(&input);
    part2(&input);
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos(i32, i32);

fn part1(input: &str) {
    let mut current_pos = Pos(0, 0);
    let mut set: HashSet<Pos> = HashSet::from([current_pos]);
    for c in input.chars() {
         match c {
            '^' => current_pos.1 -= 1,
            '>' => current_pos.0 += 1,
            'v' => current_pos.1 += 1,
            '<' => current_pos.0 -= 1,
            _ => eprintln!("Unknown character")
        }
        set.insert(current_pos);
    }
    println!("Part 1: {}", set.len());
}

fn part2(input: &str) {
    let mut santa = Pos(0, 0);
    let mut robot = Pos(0, 0);
    let mut set: HashSet<Pos> = HashSet::from([santa]);
    for chunk in input.chars().collect::<Vec<_>>().chunks(2) {
        match chunk[0] {
            '^' => santa.1 -= 1,
            '>' => santa.0 += 1,
            'v' => santa.1 += 1,
            '<' => santa.0 -= 1,
            _ => eprintln!("Unknown character")
        }
        match chunk[1] {
            '^' => robot.1 -= 1,
            '>' => robot.0 += 1,
            'v' => robot.1 += 1,
            '<' => robot.0 -= 1,
            _ => eprintln!("Unknown character")
        }
        set.insert(santa);
        set.insert(robot);
    }
    println!("Part 2: {}", set.len());
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day3.txt")
}