use std::{fs, io};
use md5::{Md5, Digest};

pub fn day4() {
    let input = read_file().expect("Unable to read file!");
    println!("Day 4 - Part 1: {}", answer(&input, 5));
    println!("Day 4 - Part 2: {}", answer(&input, 6));
}

fn answer(input: &str, nb: usize) -> i32 {
    let mut i = 0;
    loop {
        let hash = Md5::digest(input.to_owned() + i.to_string().as_str());
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