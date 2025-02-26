use std::fs;

pub fn part1() -> usize {
    let (mut bots, mut outputs) = get_initial_bots_and_outputs();
    bots.sort_by(|a, b| a.number.cmp(&b.number));
    outputs.sort_by(|a, b| a.number.cmp(&b.number));
    add_inputs_bots(&mut bots);

    let (flag, output) = process_bots(&mut bots, &mut outputs, true);
    if flag {
        output
    } else {
        0
    }
}

pub fn part2() -> u32 {
    let (mut bots, mut outputs) = get_initial_bots_and_outputs();
    bots.sort_by(|a, b| a.number.cmp(&b.number));
    outputs.sort_by(|a, b| a.number.cmp(&b.number));
    add_inputs_bots(&mut bots);

    let (flag, output) = process_bots(&mut bots, &mut outputs, false);
    if flag {
        u32::try_from(output).expect("should be a u32")
    } else {
        0
    }
}

fn process_bots(bots: &mut [Bot], outputs: &mut [Output], check_values: bool) -> (bool, usize) {
    let mut flag = false;
    let mut output = 0;

    loop {
        for i in 0..bots.len() {
            if bots[i].values.len() == 2 {
                bots[i].values.sort_unstable();
                let mut valid = true;
                if bots[i].is_lower_output_a_bot() && bots[bots[i].lower_output_number as usize].values.len() >= 2 {
                    valid = false;
                }
                if bots[i].is_higher_output_a_bot() && bots[bots[i].higher_output_number as usize].values.len() >= 2 {
                    valid = false;
                }
                if valid {
                    let value = bots[i].values[0];
                    if bots[i].is_lower_output_a_bot() {
                        let lower_output_number = bots[i].lower_output_number as usize;
                        let (left, right) = bots.split_at_mut(lower_output_number);
                        if lower_output_number < left.len() {
                            left[lower_output_number].values.push(value);
                        } else {
                            right[0].values.push(value);
                        }
                    } else {
                        outputs[bots[i].lower_output_number as usize].values.push(value);
                    }
                    let value = bots[i].values[1];
                    if bots[i].is_higher_output_a_bot() {
                        let higher_output_number = bots[i].higher_output_number as usize;
                        let (left, right) = bots.split_at_mut(higher_output_number);
                        if higher_output_number < left.len() {
                            left[higher_output_number].values.push(value);
                        } else {
                            right[0].values.push(value);
                        }
                    } else {
                        outputs[bots[i].higher_output_number as usize].values.push(value);
                    }
                    bots[i].values.clear();
                }
            }
            if check_values {
                for (j, bot) in bots.iter().enumerate() {
                    if bot.values.contains(&17) && bot.values.contains(&61) {
                        flag = true;
                        output = j;
                        break;
                    }
                }
                if flag {
                    break;
                }
            } else if !outputs[0].values.is_empty() && !outputs[1].values.is_empty() && !outputs[2].values.is_empty() {
                flag = true;
                output = (outputs[0].values[0] * outputs[1].values[0] * outputs[2].values[0]) as usize;
                break;
            }
        }

        if flag {
            break;
        }
    }

    (flag, output)
}

fn get_initial_bots_and_outputs() -> (Vec<Bot>, Vec<Output>) {
    let mut bots: Vec<Bot> = Vec::new();
    let mut outputs: Vec<Output> = Vec::new();
    for line in fs::read_to_string("resources/year2016/day10.txt").expect("Unable to read file!").lines() {
        if line.contains("value") {
            continue
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        bots.push(Bot::new(
            parts[1].parse::<u32>().unwrap(),
            parts[6].parse::<u32>().unwrap(),
            parts[11].parse::<u32>().unwrap(),
            if parts[5] == "bot" { State::Bot } else {
                outputs.push(Output::new(parts[6].parse::<u32>().unwrap()));
                State::Output
            },
            if parts[10] == "bot" { State::Bot } else {
                outputs.push(Output::new(parts[11].parse::<u32>().unwrap()));
                State::Output 
            },
        ));
    }
    (bots, outputs)
}

fn add_inputs_bots(bots: &mut [Bot]) {
    for line in fs::read_to_string("resources/year2016/day10.txt").expect("Unable to read file!").lines() {
        if !line.contains("value") {
            continue
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        bots[parts[5].parse::<u32>().unwrap() as usize].values.push(parts[1].parse::<u32>().unwrap());
    }
}

struct Bot {
    number: u32,
    values: Vec<u32>,
    lower_output_number: u32,
    higher_output_number: u32,
    lower_output_state: State,
    higher_output_state: State
}

struct Output {
    number: u32,
    values: Vec<u32>
}

impl Output {
    fn new(number: u32) -> Output {
        Output {
            number,
            values: Vec::new()
        }
    }
}

impl Bot {
    fn new(number: u32, lower_output_number: u32, higher_output_number: u32, lower_output_state: State, higher_output_state: State) -> Bot {
        Bot {
            number,
            values: Vec::new(),
            lower_output_number,
            higher_output_number,
            lower_output_state,
            higher_output_state
        }
    }
    fn is_lower_output_a_bot(&self) -> bool {
        self.lower_output_state == State::Bot
    }
    fn is_higher_output_a_bot(&self) -> bool {
        self.higher_output_state == State::Bot
    }
}

#[derive(PartialEq)]
enum State {
    Bot,
    Output,
}