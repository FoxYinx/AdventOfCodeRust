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
                    let left = left.parse::<u32>().expect("Left is not a u32");
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

pub fn part2() -> u32 {
    let input = fs::read_to_string("resources/year2016/day9.txt").expect("Unable to rad file");
    let mut decompressed_size = 0;
    let mut i = 0;

    while i < input.len() {
        let c = input.as_bytes()[i] as char;
        if c == '(' {
            let (to_skip, local_decompressed_size) = local_decompression(&input[i..input.len()]);
            i += to_skip;
            decompressed_size += local_decompressed_size;
        } else {
            decompressed_size += 1;
            i += 1;
        }
    }

    decompressed_size
}

fn local_decompression(local :&str) -> (usize, u32) {
    if let Some(right_parenthesis_pointer) = local.find(')') {
        if let Some((left, right)) = local[1..right_parenthesis_pointer].split_once('x') {
            let left = left.parse::<u32>().expect("Left is not a u32");
            let right = right.parse::<u32>().expect("Right is not a u32");

            return if local[(right_parenthesis_pointer + 1)..local.len()].contains(')') {
                let (to_skip, local_decompressed_size) = local_decompression(&local[(right_parenthesis_pointer + 1)..local.len()]);
                (right_parenthesis_pointer + 1 + left as usize + to_skip, right * local_decompressed_size)
            } else {
                (right_parenthesis_pointer + 1 + left as usize, left * right)
            }
        }
    }

    (0, 0)
}