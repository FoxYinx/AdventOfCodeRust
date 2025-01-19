use std::{fs, io};
use regex::Regex;

pub fn part1() -> i32 {
    let lines = read_lines().expect("Unable to read file!");
    let regex = Regex::new(r"capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    let ingredients: Vec<(i32, i32, i32, i32, i32)> = regex.captures_iter(&lines).map(|cap| {
        let (_, [v1, v2, v3, v4, v5]) = cap.extract();
        (v1.parse::<i32>().unwrap(), v2.parse::<i32>().unwrap(), v3.parse::<i32>().unwrap(), v4.parse::<i32>().unwrap(), v5.parse::<i32>().unwrap())
    }).collect();

    let mut best_cookie = i32::MIN;
    let combinations = find_combinations();
    for combination in combinations {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;

        for (i, &amount) in combination.iter().enumerate() {
            capacity += ingredients[i].0 * amount;
            durability += ingredients[i].1 * amount;
            flavor += ingredients[i].2 * amount;
            texture += ingredients[i].3 * amount;
        }

        capacity = capacity.max(0);
        durability = durability.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);
        
        let result = capacity * durability * flavor * texture;
        if result > best_cookie {
            best_cookie = result;
        }
    }

    best_cookie
}

pub fn part2() -> i32 {
    let lines = read_lines().expect("Unable to read file!");
    let regex = Regex::new(r"capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    let ingredients: Vec<(i32, i32, i32, i32, i32)> = regex.captures_iter(&lines).map(|cap| {
        let (_, [v1, v2, v3, v4, v5]) = cap.extract();
        (v1.parse::<i32>().unwrap(), v2.parse::<i32>().unwrap(), v3.parse::<i32>().unwrap(), v4.parse::<i32>().unwrap(), v5.parse::<i32>().unwrap())
    }).collect();

    let mut best_cookie = i32::MIN;
    let combinations = find_combinations();
    for combination in combinations {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;

        for (i, &amount) in combination.iter().enumerate() {
            capacity += ingredients[i].0 * amount;
            durability += ingredients[i].1 * amount;
            flavor += ingredients[i].2 * amount;
            texture += ingredients[i].3 * amount;
            calories += ingredients[i].4 * amount;
        }
        
        if calories != 500 {
            continue;
        }

        capacity = capacity.max(0);
        durability = durability.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);

        let result = capacity * durability * flavor * texture;
        if result > best_cookie {
            best_cookie = result;
        }
    }

    best_cookie
}

fn find_combinations() -> Vec<[i32; 4]> {
    let mut combinations = Vec::new();

    for a in 1..=97 {
        for b in 1..=98 - a {
            for c in 1..=99 - a - b {
                let d = 100 - a - b - c;
                if d >= 1 {
                    combinations.push([a, b, c, d]);
                }
            }
        }
    }

    combinations
}

fn read_lines() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day15.txt")
}