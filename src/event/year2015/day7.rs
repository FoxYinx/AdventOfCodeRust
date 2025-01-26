use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use crate::event::year2015::day7::Opcode::{And, Lshift, Not, Or, Rshift, Value};

pub fn part1() -> u16 {
    let lines = read_lines("day7.txt").expect("Unable to read file!");
    execute(&lines)
}

pub fn part2() -> u16 {
    let lines = read_lines("day7-part2.txt").expect("Unable to read file!");
    execute(&lines)
}

fn read_lines(file: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open("resources/year2015/".to_owned() + file)?).lines().collect()
}

fn line_to_opcode(lines :&Vec<String>) -> Vec<(Opcode, &str)>{
    let mut opcodes: Vec<(Opcode, &str)> = Vec::new();
    for line in lines {
        if let Some((left, right)) = line.split_once(" -> ") {
            if let Some((part1, part2)) = left.split_once(" LSHIFT ") {
                opcodes.push((Lshift(part1, part2.parse::<u16>().unwrap()), right));
            } else if let Some((part1, part2)) = left.split_once(" RSHIFT ") {
                opcodes.push((Rshift(part1, part2.parse::<u16>().unwrap()), right));
            } else if let Some((part1, part2)) = left.split_once(" OR ") {
                opcodes.push((Or(part1, part2), right));
            } else if let Some((part1, part2)) = left.split_once(" AND ") {
                opcodes.push((And(part1, part2), right));
            } else if let Some((_part1, part2)) = left.split_once("NOT ") {
                opcodes.push((Not(part2), right));
            } else {
                opcodes.push((Value(left), right));
            }
        }
    }
    opcodes
}

fn execute (lines: &Vec<String>) -> u16 {
    let opcodes = line_to_opcode(lines);
    let mut map: HashMap<&str, u16> = HashMap::new();

    loop {
        for (opcode, dest) in &opcodes {
            match opcode {
                Value(v) => if let Ok(num) = v.parse::<u16>() {
                    map.insert(dest, num);
                } else if let Some(value) = map.get(v) {
                    map.insert(dest, *value);
                },
                Not(v) => if let Ok(num) = v.parse::<u16>() {
                    map.insert(dest, !num);
                } else if let Some(value) = map.get(v) {
                    map.insert(dest, !value);
                },
                Lshift(v1, v2) => if let Ok(num) = v1.parse::<u16>() {
                    map.insert(dest, num << v2);
                } else if let Some(value) = map.get(v1) {
                    map.insert(dest, value << v2);
                },
                Rshift(v1, v2) => if let Ok(num) = v1.parse::<u16>() {
                    map.insert(dest, num >> v2);
                } else if let Some(value) = map.get(v1) {
                    map.insert(dest, value >> v2);
                },
                And(v1, v2) => if let Some(value2) = map.get(v2) {
                    if let Ok(num) = v1.parse::<u16>() {
                        map.insert(dest, num & value2);
                    } else if let Some(value) = map.get(v1) {
                        map.insert(dest, value & value2);
                    }
                },
                Or(v1, v2) => if let Some(value2) = map.get(v2) {
                    if let Ok(num) = v1.parse::<u16>() {
                        map.insert(dest, num | value2);
                    } else if let Some(value) = map.get(v1) {
                        map.insert(dest, value | value2);
                    }
                }
            }
        }

        if let Some(v) = map.get("a") {
            return *v
        }
    }
}

enum Opcode<'a> {
    Value(&'a str),
    Not(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Lshift(&'a str, u16),
    Rshift(&'a str, u16)
}