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
    update_lamps_common(lamps, &mut new_lamps);
    new_lamps
}

fn update_lamps_2(lamps: &[[bool; 100]; 100]) -> [[bool; 100]; 100] {
    let mut new_lamps: [[bool; 100]; 100] = [[false; 100]; 100];
    update_lamps_common(lamps, &mut new_lamps);
    new_lamps[0][0] = true;
    new_lamps[0][99] = true;
    new_lamps[99][0] = true;
    new_lamps[99][99] = true;
    new_lamps
}

fn update_lamps_common(lamps: &[[bool; 100]; 100], new_lamps: &mut [[bool; 100]; 100]) {
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
}

fn count_on_neighbors(lamps: &[[bool; 100]; 100], row: usize, col: usize) -> usize {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    directions.iter().filter(|&&(dx, dy)| {
        if let (Ok(new_row_isize), Ok(new_col_isize)) = (isize::try_from(row), isize::try_from(col)) {
            let new_row = new_row_isize + dx;
            let new_col = new_col_isize + dy;

            if (0..100).contains(&new_row) && (0..100).contains(&new_col) {
                if let (Ok(new_row_usize), Ok(new_col_usize)) = (usize::try_from(new_row), usize::try_from(new_col)) {
                    return lamps[new_row_usize][new_col_usize];
                }
            }
        }
        false
    }).count()
}

fn read_lines() -> io::Result<Vec<String>> {
    BufReader::new(File::open("ressources/year2015/day18.txt")?).lines().collect()
}