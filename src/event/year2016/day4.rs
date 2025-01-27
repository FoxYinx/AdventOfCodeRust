use counter::Counter;
use std::fs;
use std::iter::zip;

pub fn part1() -> u32 {
    let file = fs::read_to_string("resources/year2016/day4.txt").expect("Unable to read file!");
    let mut answer = u32::MIN;
    for line in file.lines() {
        let mut flag = true;
        let (left, right) = line.split_at(line.rfind('-').unwrap());
        let mut counter = left.chars().collect::<Counter<_>>();
        counter[&'-'] = 0;
        let (index, cheksum) = right.split_once('[').unwrap();
        zip(cheksum.chars().take(5), counter.k_most_common_ordered(5))
            .for_each(|(c, (ch, _))| {
                if c != ch {
                    flag = false;
                }
            });
        if flag {
            let index = index.chars().skip(1).collect::<String>();
            answer += index.parse::<u32>().unwrap();
        }
    }
    
    answer
}