use std::collections::HashMap;
use std::{fs, io};

pub fn part1() -> i32 {
    let goal = read_input().expect("Unable to read file!").parse::<i32>().unwrap() / 10;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..goal {
        let sum: i32 = get_divisors(i, &mut map).iter().sum();
        if sum == goal {
            return i;
        }
    }
    0
}

fn get_divisors(input: i32, map: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
    if let Some(divisors) = map.get(&input) {
        return divisors.clone();
    }

    let mut divisors = Vec::new();
    for i in (1..=input / 2).rev() {
        if !divisors.contains(&i) && input % i == 0 {
            divisors.push(i);
            let sub_divisors = get_divisors(i, map);
            for &sub_divisor in &sub_divisors {
                if !divisors.contains(&sub_divisor) {
                    divisors.push(sub_divisor);
                }
            }
        }
    }
    divisors.push(input);

    map.insert(input, divisors.clone());
    divisors
}

fn read_input() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day20.txt")
}