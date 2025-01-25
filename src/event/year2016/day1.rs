use std::collections::HashSet;
use std::fs;
use regex::Regex;

pub fn part1() -> i16 {
    let regex = Regex::new(r"(\w)(\d+)").expect("Unable to create regex!");
    let line = fs::read_to_string("ressources/year2016/day1.txt").expect("Unable to read file!");
    let mut current_pos = (0, 0);
    let mut current_direction = 0;
    regex.captures_iter(&line).for_each(|cap| {
        let direction = cap[1].to_string();
        let length = cap[2].parse::<i16>().unwrap();
        match direction.as_str() { 
            "R" => current_direction = (current_direction + 1) % 4,
            "L" => current_direction = (current_direction + 4 - 1) % 4,
            _ => eprintln!("Unknown value: {direction}")
        }
        match current_direction { 
            0 => current_pos.1 += length,
            1 => current_pos.0 += length,
            2 => current_pos.1 -= length,
            3 => current_pos.0 -= length,
            _ => eprintln!("Error, false direction: {current_direction}")
        }
    });

    i16::abs(current_pos.0) + i16::abs(current_pos.1)
}

pub fn part2() -> i16 {
    let regex = Regex::new(r"(\w)(\d+)").expect("Unable to create regex!");
    let line = fs::read_to_string("ressources/year2016/day1.txt").expect("Unable to read file!");
    let mut current_pos = (0, 0);
    let mut current_direction = 0;
    let mut set: HashSet<(i16, i16)> = HashSet::new();
    set.insert(current_pos);
    let mut flag = false;
    regex.captures_iter(&line).for_each(|cap| {
        if !flag {
            let direction = cap[1].to_string();
            let length = cap[2].parse::<i16>().unwrap();
            match direction.as_str() {
                "R" => current_direction = (current_direction + 1) % 4,
                "L" => current_direction = (current_direction + 4 - 1) % 4,
                _ => eprintln!("Unknown value: {direction}")
            }
            match current_direction {
                0 => {
                    for _i in 0..length {
                        current_pos.1 += 1;
                        if !set.insert(current_pos) {
                            flag = true;
                            break
                        }
                    }
                },
                1 => {
                    for _i in 0..length {
                        current_pos.0 += 1;
                        if !set.insert(current_pos) {
                            flag = true;
                            break
                        }
                    }
                },
                2 => {
                    for _i in 0..length {
                        current_pos.1 -= 1;
                        if !set.insert(current_pos) {
                            flag = true;
                            break
                        }
                    }
                },
                3 => {
                    for _i in 0..length {
                        current_pos.0 -= 1;
                        if !set.insert(current_pos) {
                            flag = true;
                            break
                        }
                    }
                },
                _ => eprintln!("Error, false direction: {current_direction}")
            }
        }
    });

    i16::abs(current_pos.0) + i16::abs(current_pos.1)
}