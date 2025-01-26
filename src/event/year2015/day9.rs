use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use itertools::Itertools;

pub fn part1() -> u32 {
    let (map, locations) = process_graph();
    find_distance(&map, &locations, std::cmp::Ordering::Less)
}

pub fn part2() -> u32 {
    let (map, locations) = process_graph();
    find_distance(&map, &locations, std::cmp::Ordering::Greater)
}

fn process_graph() -> (HashMap<(String, String), u32>, Vec<String>){
    let lines = read_lines().expect("Unable to read file!");
    let mut map: HashMap<(String, String), u32> = HashMap::new();
    let mut locations = Vec::new();

    for line in lines {
        if let Some((c1, right)) = line.split_once(" to ") {
            if let Some((c2, dist)) = right.split_once(" = ") {
                let dist = dist.parse::<u32>().unwrap();
                map.insert((c1.to_string(), c2.to_string()), dist);
                map.insert((c2.to_string(), c1.to_string()), dist);
                if !locations.contains(&c1.to_string()) {
                    locations.push(c1.to_string());
                }
                if !locations.contains(&c2.to_string()) {
                    locations.push(c2.to_string());
                }
            }
        }
    }
    (map, locations)
}

fn find_distance(map: &HashMap<(String, String), u32>, locations: &[String], order: std::cmp::Ordering) -> u32 {
    let mut best_distance = match order {
        std::cmp::Ordering::Less => u32::MAX,
        std::cmp::Ordering::Greater => u32::MIN,
        std::cmp::Ordering::Equal => unreachable!(),
    };

    for perm in locations.iter().permutations(locations.len()) {
        let mut distance = 0;
        for i in 0..perm.len() - 1 {
            distance += map.get(&(perm[i].clone(), perm[i + 1].clone())).unwrap();
        }
        match order {
            std::cmp::Ordering::Less => {
                if distance < best_distance {
                    best_distance = distance;
                }
            }
            std::cmp::Ordering::Greater => {
                if distance > best_distance {
                    best_distance = distance;
                }
            }
            std::cmp::Ordering::Equal => unreachable!(),
        }
    }

    best_distance
}

fn read_lines() -> io::Result<Vec<String>> {
    BufReader::new(File::open("resources/year2015/day9.txt")?).lines().collect()
}