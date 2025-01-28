use std::fs;

pub fn part1() -> u16 {
    let file = fs::read_to_string("resources/year2016/day8.txt").expect("Unable to read file!");
    let mut lamps: Vec<Vec<u8>> = vec![vec![0; 50]; 6];
    for line in file.lines() {
        if let Some((_rect, dimensions)) = line.split_once("rect ") {
            if let Some((length, height)) = dimensions.split_once('x') {
                let length = length.parse::<usize>().expect("Is not a usize");
                let height = height.parse::<usize>().expect("Is not a usize");
                for i in 0..height {
                    for j in 0..length {
                        lamps[i][j] = 1;
                    }
                }
            }
        } else if let Some((_rotate, info)) = line.split_once("rotate column x=") {
            if let Some((x, by)) = info.split_once(" by ") {
                let x = x.parse::<usize>().expect("Is not a usize");
                let by = by.parse::<usize>().expect("Is not a usize");
                let mut column: Vec<u8> = lamps.iter().map(|row| row[x]).collect();
                column.rotate_right(by);
                for (i, &val) in column.iter().enumerate() {
                    lamps[i][x] = val;
                }
            }
        } else if let Some((_rotate, info)) = line.split_once("rotate row y=") {
            if let Some((y, by)) = info.split_once(" by ") {
                let y = y.parse::<usize>().expect("Is not a usize");
                let by = by.parse::<usize>().expect("Is not a usize");
                lamps[y].rotate_right(by);
            }
        }
    }

    // Print the lamps array
    for row in &lamps {
        for &lamp in row {
            print!("{}", if lamp == 1 { '#' } else { '.' });
        }
        println!();
    }
    
    lamps.iter().flat_map(|row| row.iter()).map(|&x| u16::from(x)).sum()
}