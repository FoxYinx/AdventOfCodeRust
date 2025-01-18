use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::{fs, io};

pub fn part1() -> i32 {
    let (map, people) = process_graph();
    let mut highest_value = i32::MIN;
    
    for perm in people.iter().permutations(people.len()) {
        let mut temp_value = 0;
        for i in 0..perm.len() {
            temp_value += map.get(&(perm[i].clone(), perm[(i + 1) % perm.len()].clone())).unwrap();
            temp_value += map.get(&(perm[i].clone(), perm[(i + perm.len() - 1) % perm.len()].clone())).unwrap();
        }
        if temp_value > highest_value {
            highest_value = temp_value;
        }
    }

    highest_value
}

pub fn part2() -> i32 {
    let (mut map, mut people) = process_graph();
    let mut highest_value = i32::MIN;
    add_myself(&mut map, &mut people);

    for perm in people.iter().permutations(people.len()) {
        let mut temp_value = 0;
        for i in 0..perm.len() {
            temp_value += map.get(&(perm[i].clone(), perm[(i + 1) % perm.len()].clone())).unwrap();
            temp_value += map.get(&(perm[i].clone(), perm[(i + perm.len() - 1) % perm.len()].clone())).unwrap();
        }
        if temp_value > highest_value {
            highest_value = temp_value;
        }
    }

    highest_value
}

fn process_graph() -> (HashMap<(String, String), i32>, Vec<String>){
    let lines = read_lines().expect("Unable to read file!");
    let mut map: HashMap<(String, String), i32> = HashMap::new();
    let mut people: Vec<String> = Vec::new();
    let regex = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();
    for caps in regex.captures_iter(&lines) {
        let n1 = &caps[1];
        let gl = &caps[2];
        let value = caps[3].parse::<i32>().unwrap();
        let n2 = &caps[4];
        
        if !people.contains(&n1.to_string()) {
            people.push(n1.to_string());
        }
        if gl == "gain" {
            map.insert((n1.to_string(), n2.to_string()), value);
        } else {
            map.insert((n1.to_string(), n2.to_string()), -value);
        }
    }

    (map, people)
}

fn add_myself(map: &mut HashMap<(String, String), i32>, people: &mut Vec<String>) {
    for person in people.iter() {
        map.insert(("myself".to_string(), person.to_string()), 0);
        map.insert((person.to_string(), "myself".to_string()), 0);
    }
   people.push("myself".to_string());
}

fn read_lines() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day13.txt")
}