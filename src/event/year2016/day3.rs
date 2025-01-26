use std::fs;
use regex::Regex;

pub fn part1() -> u16 {
    let mut valid = u16::MIN;
    let lines = fs::read_to_string("ressources/year2016/day3.txt").expect("Unable to read file!");
    let regex = Regex::new(r"(\d+) *(\d+) *(\d+)").expect("Unable to make regex");
    regex.captures_iter(&lines).for_each(|cap| {
        let s1 = cap[1].parse::<u16>().unwrap();
        let s2 = cap[2].parse::<u16>().unwrap();
        let s3 = cap[3].parse::<u16>().unwrap();
        if s1 + s2 > s3 && s1 + s3 > s2 && s2 + s3 > s1 {
            valid += 1;
        }
    });
    
    valid
}