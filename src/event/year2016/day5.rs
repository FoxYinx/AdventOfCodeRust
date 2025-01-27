use std::fs;
use md5::{Digest, Md5};

pub fn part1() -> String {
    let door_id = fs::read_to_string("resources/year2016/day5.txt").expect("Unable to read file!");
    let mut output = String::new();
    for i in 0.. {
        let hash = base16ct::lower::encode_string(&Md5::digest(format!("{door_id}{i}").as_str()));
        if hash.starts_with("00000") {
            output.push(hash.chars().nth(5).unwrap());
            if output.len() == 8 {
                break;
            }
        }
    }
    output
}