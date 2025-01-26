use std::fs;

pub fn part1() -> u16 {
    let mut valid = u16::MIN;
    let file = fs::read_to_string("ressources/year2016/day3.txt").expect("Unable to read file!");
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