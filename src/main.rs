use std::fmt::Display;
use std::time::Instant;

mod event;

fn main() {
    let start = Instant::now();

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