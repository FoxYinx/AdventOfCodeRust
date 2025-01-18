use std::{fs, io};
use std::cmp::min;
use regex::Regex;

pub fn part1() -> u32 {
    let lines = read_lines().expect("Unable to read file!");
    let mut furthest = u32::MIN;
    let regex = Regex::new(r"(\d+) km/s for (\d+) seconds, but then must rest for (\d+)").unwrap();
    regex.captures_iter(&lines).for_each(|cap| {
        let (_, [speed, time, pause]) = cap.extract();
        let distance = distance(speed.parse::<u32>().unwrap(), time.parse::<u32>().unwrap(), pause.parse::<u32>().unwrap(), 2503);
        if distance > furthest {
            furthest = distance;
        }
    });
    
    furthest
}

fn distance(speed: u32, time: u32, pause: u32, after: u32) -> u32 {
    let quotient = after / (time + pause);
    let remainder = after % (time + pause);
    
    speed * time * quotient + speed * min(time, remainder)
}

fn read_lines() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day14.txt")
}