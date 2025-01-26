use regex::Regex;
use std::{fs, io};
use std::cmp::max;

pub fn part1() -> i32 {
    let values = get_values();
    
    let mut lamps: Box<[[bool; 1000]]> = vec![[false; 1000]; 1000].into_boxed_slice();
    for value in values {
        match value.0 {
            Function::Toggle => {
                for column in lamps.iter_mut().take(value.3).skip(value.1 - 1) {
                    for lamp in column.iter_mut().take(value.4).skip(value.2 - 1) {
                        *lamp = !*lamp;
                    }
                }
            },
            Function::On => {
                for column in lamps.iter_mut().take(value.3).skip(value.1 - 1) {
                    for lamp in column.iter_mut().take(value.4).skip(value.2 - 1) {
                        *lamp = true;
                    }
                }
            },
            Function::Off => {
                for column in lamps.iter_mut().take(value.3).skip(value.1 - 1) {
                    for lamp in column.iter_mut().take(value.4).skip(value.2 - 1) {
                        *lamp = false;
                    }
                }
            },
            Function::Error => eprintln!("Error while parsing input")
        }
    }

    lamps.iter().fold(0, |acc, bool| acc + bool.iter().fold(0, |acc, bool| if *bool {acc + 1} else {acc}))
}

pub fn part2() -> i32 {
    let values = get_values();
    
    let mut lamps: Box<[[i32; 1000]]> = vec![[0; 1000]; 1000].into_boxed_slice();
    for value in values {
        match value.0 {
            Function::Toggle => {
                for column in lamps.iter_mut().take(value.3).skip(value.1 - 1) {
                    for lamp in column.iter_mut().take(value.4).skip(value.2 - 1) {
                        *lamp += 2;
                    }
                }
            },
            Function::On => {
                for column in lamps.iter_mut().take(value.3).skip(value.1 - 1) {
                    for lamp in column.iter_mut().take(value.4).skip(value.2 - 1) {
                        *lamp += 1;
                    }
                }
            },
            Function::Off => {
                for column in lamps.iter_mut().take(value.3).skip(value.1 - 1) {
                    for lamp in column.iter_mut().take(value.4).skip(value.2 - 1) {
                        *lamp = max(0, *lamp - 1);
                    }
                }
            },
            Function::Error => eprintln!("Error while parsing input")
        }
    }

    lamps.iter().fold(0, |acc, val| acc + val.iter().sum::<i32>())

}

fn get_values() -> Vec<(Function, usize, usize, usize, usize)> {
    let input = read_file().expect("Unable to read file!");
    let regex = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    regex.captures_iter(&input).map(|caps| {
        let (_, [f, x1, y1, x2, y2]) = caps.extract();
        let func = match f { 
            "turn on" => Function::On,
            "turn off" => Function::Off,
            "toggle" => Function::Toggle,
            _ => Function::Error
        };
        (func, x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap(), x2.parse::<usize>().unwrap(), y2.parse::<usize>().unwrap())
    }).collect()
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("resources/year2015/day6.txt")
}

enum Function {
    Toggle,
    On,
    Off,
    Error
}