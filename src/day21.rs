use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

fn scramble_password(instructions: &str, password: &str) -> String {
    let re_swap: Regex =
        Regex::new(r"swap (position|letter) (\d+|\w) with (position|letter) (\d+|\w)").unwrap();
    let re_rotate_dir: Regex = Regex::new(r"rotate (left|right) (\d+) step").unwrap();
    let re_rotate_pos: Regex = Regex::new(r"rotate based on position of letter (\w)").unwrap();
    let re_reverse: Regex = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let re_move: Regex = Regex::new(r"move position (\d+) to position (\d+)").unwrap();

    let mut password: VecDeque<char> = password.chars().collect();
    for ins in instructions.lines() {
        for cap in re_swap.captures_iter(ins) {
            let pos_check = &cap[1] == "position";
            if pos_check {
                let x: usize = cap[2].parse().unwrap();
                let y: usize = cap[4].parse().unwrap();
                password.swap(x, y);
            } else {
                let x: char = cap[2].parse().unwrap();
                let y: char = cap[4].parse().unwrap();
                password = password
                    .iter()
                    .map(|&c| {
                        if c == x {
                            y
                        } else if c == y {
                            x
                        } else {
                            c
                        }
                    })
                    .collect();
            }
        }
        for cap in re_rotate_dir.captures_iter(ins) {
            let rotate_left = &cap[1] == "left";
            let amount: usize = cap[2].parse().unwrap();
            if rotate_left {
                password.rotate_left(amount % password.len());
            } else {
                password.rotate_right(amount % password.len());
            }
        }
        for cap in re_rotate_pos.captures_iter(ins) {
            let pivot: char = cap[1].parse().unwrap();
            let mut index = password.iter().position(|&c| c == pivot).unwrap();
            if index >= 4 {
                index += 1
            }
            password.rotate_right((1 + index) % password.len());
        }
        for cap in re_reverse.captures_iter(ins) {
            let x: usize = cap[1].parse().unwrap();
            let y: usize = cap[2].parse().unwrap();
            let mut reversed = VecDeque::new();
            for i in x..=y {
                reversed.push_front(password[i]);
            }
            for (i, &rev_elem) in reversed.iter().enumerate() {
                password[x + i] = rev_elem;
            }
        }
        for cap in re_move.captures_iter(ins) {
            let x: usize = cap[1].parse().unwrap();
            let y: usize = cap[2].parse().unwrap();
            let extracted = password.remove(x).unwrap();
            password.insert(y, extracted);
        }
    }
    password.iter().collect()
}

fn un_scramble_password(instructions: &str, goal: &str) -> String {
    for possible_solution in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
        .iter()
        .permutations(goal.len())
    {
        let input: String = possible_solution.into_iter().collect();
        let result = scramble_password(instructions, &input);
        if result == goal {
            return input;
        }
    }
    panic!("Could not find a solution..");
}

pub fn main() {
    let input = read_to_string("inputs/day21.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "fdhbcgea";
    let part_1: String = scramble_password(&input, "abcdefgh");
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "egfbcadh";
    let part_2: String = un_scramble_password(&input, "fbgdceah");
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "swap position 4 with position 0\nswap letter d with letter b\nreverse positions 0 through 4\nrotate left 1 step\nmove position 1 to position 4\nmove position 3 to position 0\nrotate based on position of letter b\nrotate based on position of letter d";
        let answer: String = scramble_password(&input, "abcde");
        assert_eq!(answer, "decab");
    }
    #[test]
    fn test_example_2() {
        let input = "swap position 4 with position 0\nswap letter d with letter b\nreverse positions 0 through 4\nrotate left 1 step\nmove position 1 to position 4\nmove position 3 to position 0\nrotate based on position of letter b\nrotate based on position of letter d";
        let answer: String = un_scramble_password(&input, "decab");
        assert_eq!(answer, "abcde");
    }
}
