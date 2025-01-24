use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn part1() -> u64 {
    let values = read_lines();
    let goal: u64 = values.iter().sum::<u64>() / 3;
    let mut answer: Vec<&u64> = Vec::new();
    
    let mut iter = 0;
    let mut sum = 0;
    for i in (0..values.len()).rev() {
        if sum < goal {
            iter += 1;
            sum += values[i];
        } else {
            break;
        }
    }

    let mut flag = false;
    for i in iter..(values.len() - 2) {
        for permutation in values.iter().permutations(i) {
            let sum: u64 = permutation.iter().copied().sum();
            if sum == goal {
                answer.clone_from(&permutation);
                flag = true;
                break;
            }
        }
        if flag {
            break;
        }
    }
    
    answer.iter().copied().product()
}

fn read_lines() -> Vec<u64> {
    let file = File::open("ressources/year2015/day24.txt").expect("Unable to read file!");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().parse().unwrap()).collect()
}