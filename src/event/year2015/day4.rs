use std::{fs, io};
use md5::{Md5, Digest};

pub fn part1() -> i32 {
    answer(5)
}

pub fn part2() -> i32 {
    answer(6)
}

fn answer(nb: usize) -> i32 {
    let input = read_file().expect("Unable to read file!");
    let mut i = 0;
    loop {
        let hash = Md5::digest(input.clone() + i.to_string().as_str());
        let hex_hash = base16ct::lower::encode_string(&hash);
        if hex_hash[..nb] == "0".repeat(nb) {
            break;
        }
        i += 1;
    }
    i
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day4.txt")
}