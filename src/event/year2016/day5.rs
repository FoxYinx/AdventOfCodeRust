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

pub fn part2() -> String {
    let door_id = fs::read_to_string("resources/year2016/day5.txt").expect("Unable to read file!");
    let mut output: [char; 8] = ['_'; 8];
    for i in 0.. {
        let hash = base16ct::lower::encode_string(&Md5::digest(format!("{door_id}{i}").as_str()));
        if hash.starts_with("00000") {
            if let Some(pos) = hash.chars().nth(5).unwrap().to_digit(10) {
                if pos < 8 {
                    let pos = pos as usize;
                    if output[pos] == '_' {
                        output[pos] = hash.chars().nth(6).unwrap();
                        if !output.contains(&'_') {
                            break;
                        }
                    }
                }
            }
        }
    }
    output.iter().collect()
}