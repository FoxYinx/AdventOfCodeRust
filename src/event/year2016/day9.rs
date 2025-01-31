use std::fs;

pub fn part1() -> u32 {
    let input = fs::read_to_string("resources/year2016/day9.txt").expect("Unable to read file!");
    let mut decompressed_size = 0;
    let mut i = 0;

    while i < input.len() {
        let c = input.as_bytes()[i] as char;

        if c == '(' {
            if let Some(right_parenthesis_pointer) = input[i..].find(')') {
                if let Some((left, right)) = input[(i + 1)..(i + right_parenthesis_pointer)].split_once('x') {
                    let left = left.parse::<u32>().expect("Left is not a usize");
                    let right = right.parse::<u32>().expect("Right is not a u32");

                    decompressed_size += left * right;

                    i += right_parenthesis_pointer + 1 + left as usize;
                    continue;
                }
            }
        } else {
            decompressed_size += 1;
        }
        i += 1;
    }

    decompressed_size
}
