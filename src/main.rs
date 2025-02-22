use std::fmt::Display;
use std::time::Instant;

mod event;

#[allow(dead_code)]
fn year2015() {
    println!("Year 2015");
    time(1, 1, event::year2015::day1::part1);
    time(1, 2, event::year2015::day1::part2);
    time(2, 1, event::year2015::day2::part1);
    time(2, 2, event::year2015::day2::part2);
    time(3, 1, event::year2015::day3::part1);
    time(3, 2, event::year2015::day3::part2);
    time(4, 1, event::year2015::day4::part1);
    time(4, 2, event::year2015::day4::part2);
    time(5, 1, event::year2015::day5::part1);
    time(5, 2, event::year2015::day5::part2);
    time(6, 1, event::year2015::day6::part1);
    time(6, 2, event::year2015::day6::part2);
    time(7, 1, event::year2015::day7::part1);
    time(7, 2, event::year2015::day7::part2);
    time(8, 1, event::year2015::day8::part1);
    time(8, 2, event::year2015::day8::part2);
    time(9, 1, event::year2015::day9::part1);
    time(9, 2, event::year2015::day9::part2);
    time(10, 1, event::year2015::day10::part1);
    time(10, 2, event::year2015::day10::part2);
    time(11, 1, event::year2015::day11::part1);
    time(11, 2, event::year2015::day11::part2);
    time(12, 1, event::year2015::day12::part1);
    time(12, 2, event::year2015::day12::part2);
    time(13, 1, event::year2015::day13::part1);
    time(13, 2, event::year2015::day13::part2);
    time(14, 1, event::year2015::day14::part1);
    time(14, 2, event::year2015::day14::part2);
    time(15, 1, event::year2015::day15::part1);
    time(15, 2, event::year2015::day15::part2);
    time(16, 1, event::year2015::day16::part1);
    time(16, 2, event::year2015::day16::part2);
    time(17, 1, event::year2015::day17::part1);
    time(17, 2, event::year2015::day17::part2);
    time(18, 1, event::year2015::day18::part1);
    time(18, 2, event::year2015::day18::part2);
    time(19, 1, event::year2015::day19::part1);
    time(19, 2, event::year2015::day19::part2);
    time(20, 1, event::year2015::day20::part1);
    time(20, 2, event::year2015::day20::part2);
    time(21, 1, event::year2015::day21::part1);
    time(21, 2, event::year2015::day21::part2);
    time(22, 1, event::year2015::day22::part1);
    time(22, 2, event::year2015::day22::part2);
    time(23, 1, event::year2015::day23::part1);
    time(23, 2, event::year2015::day23::part2);
    time(24, 1, event::year2015::day24::part1);
    time(24, 2, event::year2015::day24::part2);
    time(25, 1, event::year2015::day25::part1);
}

fn year2016() {
    println!("Year 2016");
    time(1, 1, event::year2016::day1::part1);
    time(1, 2, event::year2016::day1::part2);
    time(2, 1, event::year2016::day2::part1);
    time(2, 2, event::year2016::day2::part2);
    time(3, 1, event::year2016::day3::part1);
    time(3, 2, event::year2016::day3::part2);
    time(4, 1, event::year2016::day4::part1);
    time(4, 2, event::year2016::day4::part2);
    time(5, 1, event::year2016::day5::part1);
    time(5, 2, event::year2016::day5::part2);
    time(6, 1, event::year2016::day6::part1);
    time(6, 2, event::year2016::day6::part2);
    time(7, 1, event::year2016::day7::part1);
    time(7, 2, event::year2016::day7::part2);
    time(8, 1, event::year2016::day8::part1);
    time(8, 2, event::year2016::day8::part2);
    time(9, 1, event::year2016::day9::part1);
    time(9, 2, event::year2016::day9::part2);
    time(10, 1, event::year2016::day10::part1);
    time(10, 2, event::year2016::day10::part2);
}

fn main() {
    let start = Instant::now();

    //year2015();
    year2016();
    
    let end = Instant::now();
    println!("Total Time: {:?}", end.duration_since(start));
}

fn time<T, F: FnOnce() -> T>(day: u8, part: u8, f: F) where T: Display {
    let start = Instant::now();
    let result = f();
    let end = Instant::now();

    println!(
        "Day {:>2} Part {}: {:<15} | Time: {:?}",
        day,
        part,
        result,
        end.duration_since(start)
    );
}