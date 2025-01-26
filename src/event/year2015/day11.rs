use std::{fs, io};
use std::collections::HashSet;

pub fn part1() -> String {
    let line = read_line("day11.txt").expect("Unable to read file!");
    get_next_password(&line)
}

pub fn part2() -> String {
    let line = read_line("day11-part2.txt").expect("Unable to read file!");
    get_next_password(&line)
}

fn get_next_password(start: &str) -> String {
    let mut value = start.to_owned();
    loop {
        value = increase_string(&value);
        if second_condition(&value) && first_condition(&value) && third_condition(&value) {
            return value
        }
    }
}

fn increase_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut carry = true;

    for i in (0..chars.len()).rev() {
        if carry {
            if chars[i] == 'z' {
                chars[i] = 'a';
            } else {
                chars[i] = (chars[i] as u8 + 1) as char;
                carry = false;
            }
        }
    }
    
    chars.into_iter().collect()
}

fn first_condition(input: &str) -> bool {
    for window in input.chars().collect::<Vec<_>>().windows(3) {
        if ((window[0] as u8) == (window[1] as u8 - 1)) && ((window[1] as u8) == (window[2] as u8 - 1)) {
            return true;
        }
    }
    false
}

fn second_condition(input: &str) -> bool {
   !input.contains('i') && !input.contains('o') && !input.contains('l')
}

fn third_condition(input: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for window in input.chars().collect::<Vec<_>>().windows(2) {
        if window[0] == window[1] {
            set.insert(window[0]);
        }
    }
    set.len() > 1
}

fn read_line(file: &str) -> io::Result<String> {
    fs::read_to_string("resources/year2015/".to_owned() + file)
}