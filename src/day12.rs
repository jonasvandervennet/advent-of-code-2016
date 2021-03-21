use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

struct Instruction {
    opcode: char, // first char of instruction
    left_c: char,
    left_v: isize,
    left_reg: bool,
    right_c: char,
    right_v: isize,
}

impl Instruction {
    fn default() -> Instruction {
        Instruction {
            opcode: '*',
            left_c: '*',
            left_v: 0,
            left_reg: false,
            right_c: '*',
            right_v: 0,
        }
    }
    fn new(ins: &str) -> Instruction {
        let re_cpy_number: Regex = Regex::new(r"cpy (-?\d+) ([a-g])").unwrap();
        let re_cpy_reg: Regex = Regex::new(r"cpy ([a-g]) ([a-g])").unwrap();
        let re_jnz_number: Regex = Regex::new(r"jnz (-?\d+) (-?\d+)").unwrap();
        let re_jnz_reg: Regex = Regex::new(r"jnz ([a-g]) (-?\d+)").unwrap();

        let opcode = ins.chars().nth(0).unwrap();
        if opcode == 'c' {
            let mut instruction = Instruction::default();
            for cap in re_cpy_number.captures_iter(ins) {
                let value = cap[1].parse::<isize>().unwrap();
                let reg = cap[2].parse::<char>().unwrap();
                instruction = Instruction {
                    opcode: opcode,
                    left_c: '*',
                    left_v: value,
                    left_reg: false,
                    right_c: reg,
                    right_v: 0,
                };
            }
            for cap in re_cpy_reg.captures_iter(ins) {
                let src_reg = cap[1].parse::<char>().unwrap();
                let dst_reg = cap[2].parse::<char>().unwrap();
                instruction = Instruction {
                    opcode: opcode,
                    left_c: src_reg,
                    left_v: 0,
                    left_reg: true,
                    right_c: dst_reg,
                    right_v: 0,
                };
            }
            instruction
        } else if opcode == 'd' || opcode == 'i' {
            let reg = ins.chars().nth_back(0).unwrap();
            return Instruction {
                opcode: opcode,
                left_c: reg,
                left_v: 0,
                left_reg: true,
                right_c: '*',
                right_v: 0,
            };
        } else if opcode == 'j' {
            let mut instruction = Instruction::default();
            for cap in re_jnz_number.captures_iter(ins) {
                let value = cap[1].parse::<isize>().unwrap();
                let offset = cap[2].parse::<isize>().unwrap();
                instruction = Instruction {
                    opcode: opcode,
                    left_c: '*',
                    left_v: value,
                    left_reg: false,
                    right_c: '*',
                    right_v: offset,
                };
            }
            for cap in re_jnz_reg.captures_iter(ins) {
                let reg = cap[1].parse::<char>().unwrap();
                let offset = cap[2].parse::<isize>().unwrap();
                instruction = Instruction {
                    opcode: opcode,
                    left_c: reg,
                    left_v: 0,
                    left_reg: true,
                    right_c: '*',
                    right_v: offset,
                };
            }
            instruction
        } else {
            panic!("Unknown opcode {}", opcode);
        }
    }
    fn do_ins(&self, i: isize, registers: &mut HashMap<char, isize>) -> isize {
        match self.opcode {
            'c' => {
                let value = if self.left_reg {
                    *registers.entry(self.left_c).or_insert(0)
                } else {
                    self.left_v
                };
                registers
                    .entry(self.right_c)
                    .and_modify(|e| *e = value)
                    .or_insert(value);
                i + 1
            }
            'd' => {
                registers
                    .entry(self.left_c)
                    .and_modify(|e| *e -= 1)
                    .or_insert(-1);
                i + 1
            }
            'i' => {
                registers
                    .entry(self.left_c)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                i + 1
            }
            'j' => {
                let value = if self.left_reg {
                    *registers.entry(self.left_c).or_insert(0)
                } else {
                    self.left_v
                };
                if value != 0 {
                    i + self.right_v
                } else {
                    i + 1
                }
            }
            _ => unreachable!(),
        }
    }
}

fn process_instructions(input: &str, part: usize) -> isize {
    let instructions_string: Vec<&str> = input.lines().collect();
    let mut registers: HashMap<char, isize> = HashMap::new();
    if part == 2 {
        registers.insert('c', 1);
    }

    let instructions: Vec<Instruction> = instructions_string
        .iter()
        .map(|ins| Instruction::new(ins))
        .collect();

    let mut i: isize = 0;
    loop {
        let ins = &instructions[i as usize];
        i = ins.do_ins(i, &mut registers);
        if i < 0 || (i as usize) >= instructions.len() {
            break;
        }
    }
    registers[&'a']
}

pub fn main() {
    let input = read_to_string("inputs/day12.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "318077";
    let part_1: isize = process_instructions(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "9227731";
    let part_2: isize = process_instructions(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a";
        let answer: isize = process_instructions(&input, 1);
        assert_eq!(answer, 42);
    }
}
