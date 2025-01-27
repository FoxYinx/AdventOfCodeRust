use std::fs;
use counter::Counter;

pub fn part1() -> String {
    let file = fs::read_to_string("resources/year2016/day6.txt").expect("Unable to read file!");
    let mut counters: [Counter<char>; 8] = Default::default();

    for line in file.lines() {
        for (i, c) in line.chars().enumerate() {
            counters[i][&c] += 1;
        }
    }
    
    let result: String = counters.iter().map(|counter| {
        counter.most_common().first().unwrap().0
    }).collect();

    result
}

pub fn part2() -> String {
    let file = fs::read_to_string("resources/year2016/day6.txt").expect("Unable to read file!");
    let mut counters: [Counter<char>; 8] = Default::default();

    for line in file.lines() {
        for (i, c) in line.chars().enumerate() {
            counters[i][&c] += 1;
        }
    }

    let result: String = counters.iter().map(|counter| {
        counter.most_common().last().unwrap().0
    }).collect();

    result
}