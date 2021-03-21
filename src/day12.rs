use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;

fn process_instructions(input: &str, part: usize) -> isize {
    let instructions: Vec<&str> = input.lines().collect();
    let mut registers: HashMap<char, isize> = HashMap::new();
    if part == 2 {
        registers.insert('c', 1);
    }

    let re_cpy_number: Regex = Regex::new(r"cpy (-?\d+) ([a-g])").unwrap();
    let re_cpy_reg: Regex = Regex::new(r"cpy ([a-g]) ([a-g])").unwrap();
    let re_jnz_number: Regex = Regex::new(r"jnz (-?\d+) (-?\d+)").unwrap();
    let re_jnz_reg: Regex = Regex::new(r"jnz ([a-g]) (-?\d+)").unwrap();

    let mut i: isize = 0;
    // TODO: lot of jumps, don't parse instructions each time
    loop {
        let ins = instructions[i as usize];
        match ins.chars().nth(0).unwrap() {
            'c' => {
                for cap in re_cpy_number.captures_iter(ins) {
                    let value = cap[1].parse::<isize>().unwrap();
                    let reg = cap[2].parse::<char>().unwrap();
                    registers.entry(reg).and_modify(|e| { *e = value }).or_insert(value);
                }
                for cap in re_cpy_reg.captures_iter(ins) {
                    let src_reg = cap[1].parse::<char>().unwrap();
                    let src_value = *registers.entry(src_reg).or_insert(0);
                    let dst_reg = cap[2].parse::<char>().unwrap();
                    registers.entry(dst_reg).and_modify(|e| { *e = src_value }).or_insert(src_value);
                }
                i += 1;
            }
            'd' => {
                let reg = ins.chars().nth_back(0).unwrap();
                registers.entry(reg).and_modify(|e| { *e -= 1 }).or_insert(-1);
                i += 1;
            }
            'i' => {
                let reg = ins.chars().nth_back(0).unwrap();
                registers.entry(reg).and_modify(|e| { *e += 1 }).or_insert(1);
                i += 1;
            }
            'j' => {
                for cap in re_jnz_number.captures_iter(ins) {
                    let value = cap[1].parse::<isize>().unwrap();
                    let offset = cap[2].parse::<isize>().unwrap();
                    if value != 0 {
                        i = i.checked_add(offset).unwrap();
                    } else {
                        i += 1;
                    }
                }
                for cap in re_jnz_reg.captures_iter(ins) {
                    let reg = cap[1].parse::<char>().unwrap();
                    let offset = cap[2].parse::<isize>().unwrap();
                    let reg_value = *registers.entry(reg).or_insert(0);
                    if reg_value != 0 {
                        i = i.checked_add(offset).unwrap();
                    } else {
                        i += 1;
                    }
                }
            }
            _ => unreachable!()
        }
        if i < 0 || (i as usize) >= instructions.len() {break;}
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
