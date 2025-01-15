use regex::Regex;
use std::{fs, io};

pub fn part1() -> i32 {
    let values = get_values();
    
    let mut lamps: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    for value in values {
        match value.0 {
            Function::Toggle => {
                for x in value.1 - 1..value.3 {
                    for y in value.2 - 1..value.4 {
                        lamps[x][y] = !lamps[x][y];
                    }
                }
            },
            Function::On => {
                for x in value.1 - 1..value.3 {
                    for y in value.2 - 1..value.4 {
                        lamps[x][y] = true;
                    }
                }
            },
            Function::Off => {
                for x in value.1 - 1..value.3{
                    for y in value.2 - 1..value.4 {
                        lamps[x][y] = false;
                    }
                }
            },
            Function::Error => eprintln!("Error while parsing input")
        }
    }

    lamps.iter().fold(0, |acc, bool| acc + bool.iter().fold(0, |acc, bool| if *bool {acc + 1} else {acc}))
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
    fs::read_to_string("ressources/year2015/day6.txt")
}

enum Function {
    Toggle,
    On,
    Off,
    Error
}