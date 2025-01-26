use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const KEYPAD: &[&[&str]] = &[
    &["1", "2", "3"],
    &["4", "5", "6"],
    &["7", "8", "9"]]
;

const HELL_KEYPAD: &[&[&str]] = &[
    &["#", "#", "1", "#", "#"],
    &["#", "2", "3", "4", "#"],
    &["5", "6", "7", "8", "9"],
    &["#", "A", "B", "C", "#"],
    &["#", "#", "D", "#", "#"]]
;

fn execute(keypad: &[&[&str]], start: (usize, usize)) -> String {
    let lines = read_lines().expect("Unable to read file!");
    let mut pointer = start;
    let mut passcode: String = String::new();
    for line in lines {
        line.chars().for_each(|char| {
            match char {
                'U' => {
                    if pointer.0 != 0 && keypad[pointer.0 - 1][pointer.1] != "#" {
                        pointer.0 -= 1;
                    }
                },
                'D' => {
                    if pointer.0 != keypad.len() - 1 && keypad[pointer.0 + 1][pointer.1] != "#" {
                        pointer.0 += 1;
                    }
                },
                'L' => {
                    if pointer.1 != 0 && keypad[pointer.0][pointer.1 - 1] != "#" {
                        pointer.1 -= 1;
                    }
                },
                'R' => {
                    if pointer.1 != keypad[0].len() - 1 && keypad[pointer.0][pointer.1 + 1] != "#" {
                        pointer.1 += 1;
                    }
                },
                _ => eprintln!("Error while parsing line, found {char}")
            }
        });
        passcode += keypad[pointer.0][pointer.1];
    }

    passcode
}

pub fn part1() -> String {
    execute(KEYPAD, (1, 1))
}

pub fn part2() -> String {
    execute(HELL_KEYPAD, (2, 0))
}

fn read_lines() -> io::Result<Vec<String>> {
    BufReader::new(File::open("../../../resources/year2016/day2.txt")?).lines().collect()
}