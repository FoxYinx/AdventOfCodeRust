use std::{fs, io};
use std::collections::HashMap;
use regex::Regex;

pub fn part1() -> i32 {
    let lines = read_lines().expect("Unable to read file!");
    let regex = Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
    let aunts: Vec<_> = regex.captures_iter(&lines).map(|cap| {
        let sue = cap[1].parse::<i32>().unwrap();
        let n1 = cap[2].to_string();
        let v1 = cap[3].parse::<i32>().unwrap();
        let n2 = cap[4].to_string();
        let v2 = cap[5].parse::<i32>().unwrap();
        let n3 = cap[6].to_string();
        let v3 = cap[7].parse::<i32>().unwrap();
        (sue, n1, v1, n2, v2, n3, v3)
    }).collect();
    
    let mut truth_map: HashMap<String, i32> = HashMap::new();
    truth_map.insert("children".to_string(), 3);
    truth_map.insert("cats".to_string(), 7);
    truth_map.insert("samoyeds".to_string(), 2);
    truth_map.insert("pomeranians".to_string(), 3);
    truth_map.insert("akitas".to_string(), 0);
    truth_map.insert("vizslas".to_string(), 0);
    truth_map.insert("goldfish".to_string(), 5);
    truth_map.insert("trees".to_string(), 3);
    truth_map.insert("cars".to_string(), 2);
    truth_map.insert("perfumes".to_string(), 1);

    for aunt in aunts {
        if truth_map[&aunt.1] == aunt.2 && truth_map[&aunt.3] == aunt.4 && truth_map[&aunt.5] == aunt.6 {
            return aunt.0;
        }
    }
    -1
}

fn read_lines() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day16.txt")
}