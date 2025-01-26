use std::{fs, io};
use md5::{Md5, Digest};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn part1() -> i32 {
    answer(5)
}

pub fn part2() -> i32 {
    answer(6)
}

fn answer(nb: usize) -> i32 {
    let input = read_file().expect("Unable to read file!");
    (0..1_500_000).into_par_iter().find_first(|&i| {
        let hash = Md5::digest(input.clone() + i.to_string().as_str());
        let hex_hash = base16ct::lower::encode_string(&hash);
        hex_hash[..nb] == "0".repeat(nb)
    }).expect("No valid hash found")
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("resources/year2015/day4.txt")
}