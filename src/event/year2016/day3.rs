use std::fs;
use itertools::Itertools;

pub fn part1() -> u16 {
    let mut valid = u16::MIN;
    let file = fs::read_to_string("../../../resources/year2016/day3.txt").expect("Unable to read file!");
    for line in file.lines() {
        let parts: Vec<u16> = line.split_whitespace().map(|s| {
            s.parse().unwrap()
        }).collect();
        if parts[0] + parts[1] > parts[2] && parts[0] + parts[2] > parts[1] && parts[1] + parts[2] > parts[0] {
            valid += 1;
        }
    }
    
    valid
}

pub fn part2() -> u16 {
    let mut valid = u16::MIN;
    let file = fs::read_to_string("../../../resources/year2016/day3.txt").expect("Unable to read file!");
    for lines in file.lines().collect::<Vec<_>>().chunks(3) {
        let part1: Vec<u16> = lines[0].split_whitespace().map(|s| {
            s.parse().unwrap()
        }).collect();
        let part2: Vec<u16> = lines[1].split_whitespace().map(|s| {
            s.parse().unwrap()
        }).collect();
        let part3: Vec<u16> = lines[2].split_whitespace().map(|s| {
            s.parse().unwrap()
        }).collect();
        for i in 0..=2 {
            if part1[0] + part2[0] > part3[0] && part1[0] + part3[0] > part2[0] && part2[0] + part3[0] > part1[0] {
                valid += 1;
            }
        }
    }

    valid
}