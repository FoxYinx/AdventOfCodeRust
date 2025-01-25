use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const KEYPAD: [[&str; 3]; 3] = [
    ["1", "2", "3"],
    ["4", "5", "6"],
    ["7", "8", "9"]]
;

const HELL_KEYPAD: [[&str; 5]; 5] = [
    ["#", "#", "1", "#", "#"],
    ["#", "2", "3", "4", "#"],
    ["5", "6", "7", "8", "9"],
    ["#", "A", "B", "C", "#"],
    ["#", "#", "D", "#", "#"]]
;

pub fn part1() -> String {
    let lines = read_lines().expect("Unable to read file!");
    let mut pointer = (1, 1);
    let mut passcode: String = String::new();
    for line in lines {
        line.chars().for_each(|char| {
            match char { 
                'U' => {
                    if pointer.0 != 0 {
                        pointer.0 -= 1;
                    }
                },
                'D' => {
                    if pointer.0 != 2 {
                        pointer.0 += 1;
                    }
                },
                'L' => {
                    if pointer.1 != 0 {
                        pointer.1 -= 1;
                    }
                },
                'R' => {
                    if pointer.1 != 2 {
                        pointer.1 += 1;
                    }
                },
                _ => eprintln!("Error while parsing line, found {char}")
            }
        });
        passcode += KEYPAD[pointer.0][pointer.1];
    }
    
    passcode
}

pub fn part2() -> String {
    let lines = read_lines().expect("Unable to read file!");
    let mut pointer = (2, 0);
    let mut passcode: String = String::new();
    for line in lines {
        line.chars().for_each(|char| {
            match char {
                'U' => {
                    if pointer.0 != 0 && HELL_KEYPAD[pointer.0 - 1][pointer.1] != "#" {
                        pointer.0 -= 1;
                    }
                },
                'D' => {
                    if pointer.0 != 4 && HELL_KEYPAD[pointer.0 + 1][pointer.1] != "#" {
                        pointer.0 += 1;
                    }
                },
                'L' => {
                    if pointer.1 != 0 && HELL_KEYPAD[pointer.0][pointer.1 - 1] != "#" {
                        pointer.1 -= 1;
                    }
                },
                'R' => {
                    if pointer.1 != 4 && HELL_KEYPAD[pointer.0][pointer.1 + 1] != "#" {
                        pointer.1 += 1;
                    }
                },
                _ => eprintln!("Error while parsing line, found {char}")
            }
        });
        passcode += HELL_KEYPAD[pointer.0][pointer.1];
    }

    passcode
}

fn read_lines() -> io::Result<Vec<String>> {
    BufReader::new(File::open("ressources/year2016/day2.txt")?).lines().collect()
}