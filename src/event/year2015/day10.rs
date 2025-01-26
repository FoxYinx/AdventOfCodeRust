use std::{fs, io};

pub fn part1() -> usize {
    let mut seq = read_file().expect("Unable to read file!");

    for _i in 0..40 {
        seq = conway(&seq);
    }

    seq.len()
}

pub fn part2() -> usize {
    let mut seq = read_file().expect("Unable to read file!");

    for _i in 0..50 {
        seq = conway(&seq);
    }
    
    seq.len()
}



fn conway(in_seq: &[i8]) -> Vec<i8> {
    let mut result = Vec::new();
    let mut current_number = in_seq[0];
    let mut current_runlength = 1;

    for i in &in_seq[1..] {
        if current_number == *i {
            current_runlength += 1;
        } else {
            result.push(current_runlength);
            result.push(current_number);
            current_runlength = 1;
            current_number = *i;
        }
    }
    result.push(current_runlength);
    result.push(current_number);
    result
}

fn read_file() -> io::Result<Vec<i8>> {
    let content = fs::read_to_string("../../../resources/year2015/day10.txt")?;
    let vec: Vec<i8> = content.trim().chars().map(|c| c.to_digit(10).unwrap().try_into().expect("Value out of range for i8")).collect();
    Ok(vec)
}