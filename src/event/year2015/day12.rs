use std::{fs, io};
use regex::Regex;
use serde_json::Value;

pub fn part1() -> i32 {
    let line = read_file().expect("Unable to read file!");
    let regex = Regex::new(r"(-?\d+)").unwrap();
    let answer: i32 = regex.captures_iter(&line).map(|num| {
        num[1].parse::<i32>().unwrap()
    }).sum();

    answer
}

pub fn part2() -> i64 {
    let line = read_file().expect("Unable to read file!");
    let data: Value = serde_json::from_str(line.as_str()).expect("Unvalid json");
    calculate_sum_without_red(&data)
}

fn calculate_sum_without_red(data: &Value) -> i64 {
    match data {
        Value::Number(num) => num.as_i64().unwrap_or(0),
        Value::Array(array) => array.iter().map(calculate_sum_without_red).sum(),
        Value::Object(obj) => {
            if obj.values().any(|v| v == "red") {
                return 0;
            }
            obj.values().map(calculate_sum_without_red).sum()
        }
        _ => 0
    }
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("resources/year2015/day12.txt")
}