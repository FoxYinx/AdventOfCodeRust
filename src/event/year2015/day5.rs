use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

pub fn part1() -> i32 {
    let input = &read_file().expect("Unable to read file!");
    let mut nice_string = 0;
    for line in input {
        if is_string_nice_1(line) {
            nice_string += 1;
        }
    }
    nice_string
}

pub fn part2() -> i32 {
    let input = &read_file().expect("Unable to read file!");
    let mut nice_string = 0;
    for line in input {
        if is_string_nice_2(line) {
            nice_string += 1;
        }
    }
    nice_string
}

fn is_string_nice_1(line: &str) -> bool {
    let mut vowels = 0;
    let mut repeat = false;
    let mut bad_strings = true;
    let chars: Vec<char> = line.chars().collect();

    if chars[0] == 'a' || chars[0] == 'e' || chars[0] == 'i' || chars[0] == 'o' || chars[0] == 'u' {
        vowels += 1;
    }

    for window in chars.windows(2) {
        match window {
            ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => bad_strings = false,
            _ => {}
        }
        if let [_, 'a' | 'e' | 'i' | 'o' | 'u'] = window {
            vowels += 1;
        }
        if window[0] == window[1] {
            repeat = true;
        }
    }

    vowels >= 3 && repeat && bad_strings
}

fn is_string_nice_2(line: &str) -> bool {
    let mut pair = false;
    let mut repeat = false;
    let chars: Vec<char> = line.chars().collect();

    for window in chars.windows(3) {
        if window[0] == window[2] {
            repeat = true;
        }
    }

    for window in chars.windows(2) {
        let pair_str: String = window.iter().collect();
        if line.matches(&pair_str).count() > 1 {
            pair = true;
        }
    }
    
    pair && repeat
}

fn read_file() -> io::Result<Vec<String>> {
    BufReader::new(File::open("ressources/year2015/day5.txt")?).lines().collect()
}