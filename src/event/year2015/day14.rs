use std::{fs, io};
use std::cmp::{min, Ordering};
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

pub fn part2() -> u32 {
    let lines = read_lines().expect("Unable to read file!");
    let mut scores = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let regex = Regex::new(r"(\d+) km/s for (\d+) seconds, but then must rest for (\d+)").unwrap();
    let reindeers: Vec<(u32, u32, u32)> = regex.captures_iter(&lines).map(|cap| {
        let (_, [speed, time, pause]) = cap.extract();
        (speed.parse::<u32>().unwrap(), time.parse::<u32>().unwrap(), pause.parse::<u32>().unwrap())
    }).collect();

    for i in 1..=2503 {
        let mut furthest = u32::MIN;
        let mut pointers: Vec<usize> = vec![];
        for reindeer in reindeers.iter().enumerate() {
            let distance = distance(reindeer.1.0, reindeer.1.1, reindeer.1.2, i);
            match distance.cmp(&furthest) {
                Ordering::Greater => {
                    furthest = distance;
                    pointers.clear();
                    pointers.push(reindeer.0);
                }
                Ordering::Equal => {
                    pointers.push(reindeer.0);
                }
                Ordering::Less => {}
            }
        }
        for pointer in &pointers {
            scores[*pointer] += 1;
        }
        pointers.clear();
    }

    *scores.iter().max().unwrap()
}

fn distance(speed: u32, time: u32, pause: u32, after: u32) -> u32 {
    let quotient = after / (time + pause);
    let remainder = after % (time + pause);

    speed * (time * quotient + min(time, remainder))
}

fn read_lines() -> io::Result<String> {
    fs::read_to_string("../../../resources/year2015/day14.txt")
}