use std::{fs, io};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos(i32, i32);

pub fn part1() -> usize {
    let input = read_file().expect("Unable to read file!");
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
    set.len()
}

pub fn part2() -> usize {
    let input = read_file().expect("Unable to read file!");
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
    set.len()
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day3.txt")
}