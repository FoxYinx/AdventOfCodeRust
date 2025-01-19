use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn part1() -> usize {
    let mut lamps = get_lamps();
    for _i in 0..100 {
        lamps = update_lamps(&lamps);
    }
    lamps.iter().map(|row| row.iter().filter(|value| **value).count()).sum()
}

pub fn part2() -> usize {
    let mut lamps = get_lamps();
    lamps[0][0] = true;
    lamps[0][99] = true;
    lamps[99][0] = true;
    lamps[99][99] = true;
    for _i in 0..100 {
        lamps = update_lamps_2(&lamps);
    }
    lamps.iter().map(|row| row.iter().filter(|value| **value).count()).sum()
}

fn get_lamps() -> [[bool; 100]; 100] {
    let lines = read_lines().expect("Unable to read file!");
    let mut lamps: [[bool; 100]; 100] = [[false; 100]; 100];
    for (row_idx, line) in lines.iter().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            if char == '#' {
                lamps[row_idx][col_idx] = true;
            }
        }
    }
    lamps
}

fn update_lamps(lamps: &[[bool; 100]; 100]) -> [[bool; 100]; 100] {
    let mut new_lamps: [[bool; 100]; 100] = [[false; 100]; 100];
    for (row_idx, lamp) in lamps.iter().enumerate() {
        for (col_idx, unique_lamp) in lamp.iter().enumerate() {
            match unique_lamp { 
                true => {
                    if (2..4).contains(&count_on_neighbors(lamps, row_idx, col_idx)) {
                        new_lamps[row_idx][col_idx] = true;
                    }
                },
                false => {
                    if count_on_neighbors(lamps, row_idx, col_idx) == 3 {
                        new_lamps[row_idx][col_idx] = true;
                    }
                }
            }
        }
    }
    new_lamps
}

fn update_lamps_2(lamps: &[[bool; 100]; 100]) -> [[bool; 100]; 100] {
    let mut new_lamps: [[bool; 100]; 100] = [[false; 100]; 100];
    for (row_idx, lamp) in lamps.iter().enumerate() {
        for (col_idx, unique_lamp) in lamp.iter().enumerate() {
            match unique_lamp {
                true => {
                    if (2..4).contains(&count_on_neighbors(lamps, row_idx, col_idx)) {
                        new_lamps[row_idx][col_idx] = true;
                    }
                },
                false => {
                    if count_on_neighbors(lamps, row_idx, col_idx) == 3 {
                        new_lamps[row_idx][col_idx] = true;
                    }
                }
            }
        }
    }
    new_lamps[0][0] = true;
    new_lamps[0][99] = true;
    new_lamps[99][0] = true;
    new_lamps[99][99] = true;
    new_lamps
}

fn count_on_neighbors(lamps: &[[bool; 100]; 100], row: usize, col: usize) -> usize {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    directions.iter().filter(|&&(dx, dy)| {
        let new_row = row as isize + dx;
        let new_col = col as isize + dy;
        (0..100).contains(&new_row) && (0..100).contains(&new_col) && lamps[new_row as usize][new_col as usize]
    }).count()
}

fn read_lines() -> io::Result<Vec<String>> {
    BufReader::new(File::open("ressources/year2015/day18.txt")?).lines().collect()
}