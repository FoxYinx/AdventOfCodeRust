use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{fs, io};

pub fn part1() -> usize {
    let lines = read_lines().expect("Unable to read file!");
    let regex = Regex::new(r"(\w+) => (\w+)").unwrap();
    let mut replacements: Vec<(String, String)> = Vec::new();
    regex.captures_iter(&lines).for_each(|cap| {
        let key = cap[1].to_string();
        let value = cap[2].to_string();
        replacements.push((key, value));
    });
    let initial_molecule = get_initial_molecule().expect("Unable to get last line!");

    let mut distinct_molecules: HashSet<String> = HashSet::new();
    for (key, value) in &replacements {
        let key_regex = Regex::new(key).unwrap();
        for mat in key_regex.find_iter(&initial_molecule) {
            let mut new_molecule = initial_molecule.clone();
            new_molecule.replace_range(mat.range(), value);
            distinct_molecules.insert(new_molecule);
        }
    }
    
    distinct_molecules.len()
}

pub fn part2() -> usize {
    let lines = read_lines().expect("Unable to read file!");
    let regex = Regex::new(r"(\w+) => (\w+)").unwrap();
    let mut replacements: Vec<(String, String)> = Vec::new();
    regex.captures_iter(&lines).for_each(|cap| {
        let lhs = cap[1].to_string();
        let rhs = cap[2].to_string();
        replacements.push((lhs, rhs));
    });
    let mut start = get_initial_molecule().expect("Unable to get last line!");
    let mut count = 0;
    loop {
        for (key, value) in &replacements {
            if let Some(pos) = start.find(value) {
                let (left, right) = start.split_at(pos);
                let right = right.to_string().split_off(value.len());
                start = format!("{left}{key}{right}");
                count += 1;
            }
        }
        if start == "e" {
            return count;
        }
    }
}


fn read_lines() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day19.txt")
}

fn get_initial_molecule() -> io::Result<String> {
    let file = File::open("ressources/year2015/day19.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    lines.last().cloned().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "File is empty"))
}