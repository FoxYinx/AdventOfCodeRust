use std::{fs, io};

pub fn part1() -> i32 {
    let goal = read_input().expect("Unable to read file!").parse::<i32>().unwrap();
    find_lowest_house(goal)
}

fn find_lowest_house(goal: i32) -> i32 {
    let goal = goal / 10;
    let mut houses = vec![0; goal as usize];

    for elf in 1..goal {
        for house in (elf..goal).step_by(elf as usize) {
            houses[house as usize] += elf * 10;
        }
    }

    for (i, &presents) in houses.iter().enumerate() {
        if presents >= goal * 10 {
            return i as i32;
        }
    }

    0
}

fn read_input() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day20.txt")
}