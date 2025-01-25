use std::fs;
use regex::Regex;

pub fn part1() -> u64 {
    let (row, column) = get_goal();
    let mut code: u64 = 20_151_125;
    for _i in 1..get_code_nb(row, column) {
        code = next_code(code);
    }
    code
}

fn get_code_nb(row: u32, column: u32) -> u32 {
    let start_value = row + column - 1;
    ((start_value + 1) * start_value) / 2 - row + 1
}

fn next_code(code: u64) -> u64 {
    (code * 252_533) % 33_554_393
}

fn get_goal() -> (u32, u32) {
    let line = fs::read_to_string("ressources/year2015/day25.txt").expect("Unable to read file!");
    let regex = Regex::new(r"(\d+), column (\d+)").unwrap();
    let caps = regex.captures(&line).unwrap();
    let row = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let column = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
    (row, column)
}