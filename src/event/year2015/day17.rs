use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn part1() -> usize {
    let values = read_lines().expect("Unable to read files");
    let mut anwser = 0;
    for value in 1..=values.len() {
        anwser += values.iter().combinations(value).filter(|vec| vec.iter().copied().sum::<i32>() == 150).count();
    }
    anwser
}

pub fn part2() -> usize {
    let values = read_lines().expect("Unable to read files");
    for value in 1..=values.len() {
        let answer= values.iter().combinations(value).filter(|vec| vec.iter().copied().sum::<i32>() == 150).count();
        if answer != 0 {
            return answer;
        }
    }
    0
}

fn read_lines() -> io::Result<Vec<i32>> {
    BufReader::new(File::open("../../../resources/year2015/day17.txt")?)
        .lines()
        .map(|line| line.and_then(|line| line.trim().parse::<i32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))))
        .collect()
}