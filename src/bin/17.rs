use std::{fmt::Display, ops::BitXor};

advent_of_code::solution!(17);

#[derive(Debug)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u64>,
    program_str: String,
    output: Vec<String>,
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A {} B {} C {}",
            self.register_a, self.register_b, self.register_c
        )
    }
}

fn bitwise_xor(value1: u64, value2: u64) -> u64 {
    value1.bitxor(value2)
}

impl Computer {
    fn output_to_string(&self) -> String {
        self.output.join(",")
    }

    fn run(&mut self, instruction_index: u64) -> u64 {
        let instruction = self.program[instruction_index as usize];
        let operand = self.program[(instruction_index + 1) as usize];
        match instruction {
            0 => {
                let operand_ = self.combo_operand(operand);
                if self.register_a <= operand_ {
                    self.register_a = 0;
                } else {
                    self.register_a = self.register_a / 2_u64.pow(operand_.try_into().unwrap());
                }
            }
            1 => self.register_b = bitwise_xor(self.register_b, operand),
            2 => self.register_b = (self.combo_operand(operand) % 8).into(),
            3 => {
                if self.register_a != 0 {
                    return operand;
                }
            }
            4 => self.register_b = bitwise_xor(self.register_b, self.register_c),
            5 => self
                .output
                .push(format!("{}", self.combo_operand(operand) % 8)),
            6 => {
                let operand_ = self.combo_operand(operand);
                if self.register_a <= operand_ {
                    self.register_b = 0;
                } else {
                    self.register_b = self.register_a / 2_u64.pow(operand_.try_into().unwrap());
                }
            }
            7 => {
                let operand_ = self.combo_operand(operand);
                if self.register_a <= operand_ {
                    self.register_c = 0;
                } else {
                    self.register_c = self.register_a / 2_u64.pow(operand_.try_into().unwrap());
                }
            }
            _ => panic!("Unknown instruction {}", instruction),
        }
        instruction_index + 2
    }

    fn combo_operand(&self, operand: u64) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Unknown operand {}", operand),
        }
    }

    fn program_index_isvalid(&self, index: u64) -> bool {
        if index >= self.program.len().try_into().unwrap() {
            false
        } else {
            true
        }
    }
}

fn parse_input(input: &str) -> Computer {
    let lines = input.lines();
    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut instructions = vec![];
    let mut program_str = "";
    for line in lines {
        let content = line.split(": ").last().unwrap();
        if line.starts_with("Register A") {
            register_a = u64::from_str_radix(content, 10).unwrap();
        }
        if line.starts_with("Register B") {
            register_b = u64::from_str_radix(content, 10).unwrap();
        }
        if line.starts_with("Register C") {
            register_c = u64::from_str_radix(content, 10).unwrap();
        }
        if line.starts_with("Program") {
            program_str = content;
            instructions = content
                .split(",")
                .map(|s| u64::from_str_radix(s, 10).unwrap())
                .collect()
        }
    }
    Computer {
        register_a,
        register_b,
        register_c,
        output: vec![],
        program: instructions,
        program_str: program_str.into(),
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = parse_input(input);
    computer = run_computer(computer);
    Some(computer.output_to_string())
}

fn run_computer(mut computer: Computer) -> Computer {
    let mut index = 0;
    while computer.program_index_isvalid(index) {
        index = computer.run(index);
    }
    computer
}

// Reverse the calculator?
pub fn part_two(input: &str) -> Option<u64> {
    let mut computer = parse_input(input);
    let mut out = 0;
    let last_i = 5 * 8_u64.pow(15) + 6 * 8_u64.pow(14) + 6 * 8_u64.pow(11) + 4 * 8_u64.pow(10);
    let len = computer.program_str.len();
    let mut out_to_search = 0;
    let mut i = 0o4025052;
    let stepsize = 8_u64.pow(7);
    println!("{}", computer.program_str);
    //                                    2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0
    // lb 31 digits 35190000000000
    // 5600640132025052 202365997689386 - 2,4,1,1,7,5,4,4,5,5,0,3,5,5,3,0
    // 5600642532025052 202366333233706 - 2,4,1,1,7,5,4,4,1,3,0,3,5,5,3,0
    // 5600657664025052 202368101657130 - 2,4,1,1,7,5,4,4,1,4,3,3,5,5,3,0
    // 5601726274025052 202441990613546 - 2,4,1,1,7,5,4,4,1,4,0,2,5,5,3,0
    // 5611726274025052 202991746427434 - 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0
    while out == 0 {
        computer.register_a = i + last_i;
        computer.output = vec![];
        let mut index = 0;
        while computer.program_index_isvalid(index) {
            if computer.output.len() > computer.program.len() {
                break;
            }
            index = computer.run(index);
        }

        if computer
            .output_to_string()
            .starts_with(&computer.program_str[0..out_to_search])
        {
            println!(
                "{:o} {} - {}",
                i + last_i,
                i + last_i,
                computer.output_to_string()
            );
            if out_to_search + 1 < len {
                out_to_search += 2;
            } else {
                out = i + last_i;
                break;
            }
        } else {
            i += stepsize;
        }
        if computer.output_to_string().len() > len {
            break;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
