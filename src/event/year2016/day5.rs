use std::fs;
use md5::{Digest, Md5};
use rayon::prelude::*;

pub fn part1() -> String {
    let door_id = fs::read_to_string("resources/year2016/day5.txt").expect("Unable to read file!");
    let output = (0..8_500_000)
        .into_par_iter()
        .map(|i| base16ct::lower::encode_string(&Md5::digest(format!("{door_id}{i}").as_str())))
        .filter(|hash| hash.starts_with("00000"))
        .fold(String::new, |mut acc, hash| {
            if acc.len() < 8 {
                acc.push(hash.chars().nth(5).unwrap());
            }
            acc
        })
        .reduce(String::new, |mut acc, item| {
            if acc.len() < 8 {
                acc.push_str(&item);
            }
            acc
        });
    output.chars().take(8).collect()
}

pub fn part2() -> String {
    let door_id = fs::read_to_string("resources/year2016/day5.txt").expect("Unable to read file!");
    let output = (0..26_000_000)
        .into_par_iter()
        .map(|i| {
            let hash = base16ct::lower::encode_string(&Md5::digest(format!("{door_id}{i}").as_str()));
            if hash.starts_with("00000") {
                if let Some(pos) = hash.chars().nth(5).unwrap().to_digit(10) {
                    if pos < 8 {
                        return Some((pos as usize, hash.chars().nth(6).unwrap()));
                    }
                }
            }
            None
        })
        .filter_map(|x| x)
        .fold(|| ['_'; 8], |mut acc, (pos, ch)| {
            if acc[pos] == '_' {
                acc[pos] = ch;
            }
            acc
        })
        .reduce(|| ['_'; 8], |mut acc, item| {
            for (i, &ch) in item.iter().enumerate() {
                if acc[i] == '_' {
                    acc[i] = ch;
                }
            }
            acc
        });

    output.iter().collect()
}