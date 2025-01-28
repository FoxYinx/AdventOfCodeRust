use std::collections::HashSet;
use std::fs;

pub fn part1() -> u32 {
    let mut valids = u32::MIN;
    let file = fs::read_to_string("resources/year2016/day7.txt").expect("Unable to read file");
    for line in file.lines() {
        let mut valid = false;
        let mut in_brackets = false;
        for window in line.as_bytes().windows(4) {
            if in_brackets {
                in_brackets = !window.contains(&93);
            } else {
                in_brackets = window.contains(&91);
            }
            if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
                if in_brackets {
                    valid = false;
                    break;
                }
                valid = true;
            }
        }
        if valid {
            valids += 1;
        }
    }
    valids
}

pub fn part2() -> u32 {
    let mut valids = u32::MIN;
    let file = fs::read_to_string("resources/year2016/day7.txt").expect("Unable to read file!");
    for line in file.lines() {
        let mut valid = false;
        let mut in_brackets = false;
        let mut outer_bracket_set: HashSet<&[u8]> = HashSet::new();
        let mut inner_bracket_set: HashSet<&[u8]> = HashSet::new();
        for window in line.as_bytes().windows(3) {
            if in_brackets {
                in_brackets = !window.contains(&93);
            } else {
                in_brackets = window.contains(&91);
            }
            if window[0] == window[2] && window[0] != window[1] {
                if in_brackets {
                    inner_bracket_set.insert(window);
                } else {
                    outer_bracket_set.insert(window);
                }
            }
        }
        for aba in inner_bracket_set {
            if valid {
                break;
            }
            for &bab in &outer_bracket_set {
                if aba[0] == bab[1] && aba[1] == bab[0] {
                    valid = true;
                }
            }
        }
        if valid {
            valids += 1;
        }
    }
    
    valids
}