use std::{fs, io};
use regex::Regex;

struct Computer {
    pointer: usize,
    instructions: Vec<Instruction>,
    registers: [u32; 2],
}

impl Computer {
    fn new () -> Computer {
        Computer {
            pointer: 0,
            instructions: Vec::new(),
            registers: [0; 2],
        }
    }

    fn execute(&mut self) -> u32 {
        while self.pointer < self.instructions.len() {
            match self.instructions.get(self.pointer).unwrap() {
                Instruction::Hlf(r) => {
                    self.registers[*r as usize] /= 2;
                    self.pointer += 1;
                },
                Instruction::Tpl(r) => {
                    self.registers[*r as usize] *= 3;
                    self.pointer += 1;
                },
                Instruction::Inc(r) => {
                    self.registers[*r as usize] += 1;
                    self.pointer += 1;
                },
                Instruction::Jmp(off) => {
                    self.pointer = (i32::try_from(self.pointer).unwrap() + i32::from(*off)).try_into().unwrap();
                },
                Instruction::Jie(r, off) => {
                    if self.registers[*r as usize] % 2 == 0 {
                        self.pointer = (i32::try_from(self.pointer).unwrap() + i32::from(*off)).try_into().unwrap();
                    } else {
                        self.pointer += 1;
                    }
                },
                Instruction::Jio(r, off) => {
                    if self.registers[*r as usize] == 1 {
                        self.pointer = (i32::try_from(self.pointer).unwrap() + i32::from(*off)).try_into().unwrap();
                    } else {
                        self.pointer += 1;
                    }
                },
                Instruction::Error => {
                    println!("Error");
                    break;
                }
            }
        }

        self.registers[1]
    }
}

pub fn part1() -> u32 {
    let mut computer = Computer::new();
    computer.instructions = get_instructions();
    computer.execute()
}

pub fn part2() -> u32 {
    let mut computer = Computer::new();
    computer.instructions = get_instructions();
    computer.registers[0] = 1;
    computer.execute()
}

fn get_instructions() -> Vec<Instruction> {
    let file = read_file().expect("Unable to read file!");
    let regex = Regex::new(r"(\w{3}) (\w)?,? ?\+?(-?\d+)?").unwrap();
    regex.captures_iter(&file).map(|caps| {
        let f = caps.get(1).unwrap().as_str();
        let mut r_or_off = caps.get(2).map_or("", |m| m.as_str());
        if r_or_off == "a" {
            r_or_off = "0";
        } else if r_or_off == "b" {
            r_or_off = "1";
        }
        let null_or_off = caps.get(3).map_or("", |m| m.as_str());
        match f {
            "hlf" => Instruction::Hlf(r_or_off.parse::<u8>().unwrap()),
            "tpl" => Instruction::Tpl(r_or_off.parse::<u8>().unwrap()),
            "inc" => Instruction::Inc(r_or_off.parse::<u8>().unwrap()),
            "jmp" => Instruction::Jmp(null_or_off.parse::<i8>().unwrap()),
            "jie" => Instruction::Jie(r_or_off.parse::<u8>().unwrap(), null_or_off.parse::<i8>().unwrap()),
            "jio" => Instruction::Jio(r_or_off.parse::<u8>().unwrap(), null_or_off.parse::<i8>().unwrap()),
            _ => Instruction::Error
        }
    }).collect()
}

fn read_file() -> io::Result<String> {
    fs::read_to_string("ressources/year2015/day23.txt")
}

enum Instruction {
    Hlf(u8),
    Tpl(u8),
    Inc(u8),
    Jmp(i8),
    Jie(u8, i8),
    Jio(u8, i8),
    Error
}