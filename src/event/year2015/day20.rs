use std::{fs, io};

pub fn part1() -> u32 {
    let goal = read_input().expect("Unable to read file!").parse::<u32>().unwrap();
    find_lowest_house(goal)
}

pub fn part2() -> u32 {
    let goal = read_input().expect("Unable to read file!").parse::<u32>().unwrap();
    find_lowest_house_2(goal)
}

fn find_lowest_house(goal: u32) -> u32 {
    let goal = goal / 10;
    let mut houses = vec![0; goal as usize];

    for elf in 1..goal {
        for house in (elf..goal).step_by(elf as usize) {
            houses[house as usize] += elf * 10;
        }
    }

    for (i, &presents) in houses.iter().enumerate() {
        if presents >= goal * 10 {
            return u32::try_from(i).unwrap();
        }
    }

    0
}

fn find_lowest_house_2(goal: u32) -> u32 {
    let goal = goal / 11;
    let mut houses = vec![0; goal as usize];

    for elf in 1..goal {
        for i in 0..50 {
            if (elf * (1 + i)) as usize >= houses.len() {
                break
            }
            houses[(elf * (1 + i)) as usize] += elf * 11;
        }
    }

    for (i, &presents) in houses.iter().enumerate() {
        if presents >= goal * 11 {
            return u32::try_from(i).unwrap();
        }
    }

    0
}

fn read_input() -> io::Result<String> {
    fs::read_to_string("../../../resources/year2015/day20.txt")
}