use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn add_value_to_tuple(old: (usize, usize), new: usize) -> (usize, usize) {
    if new < old.0 {
        (new, old.0)
    } else if new < old.1 {
        (new, old.1)
    } else {
        (old.1, new)
    }
}

fn remove_value_from_tuple(old: (usize, usize), popped_index: usize) -> (usize, usize) {
    if popped_index == 0 {
        (0, old.1)
    } else {
        (0, old.0)
    }
}

fn is_bot_full(bot: (usize, usize)) -> bool {
    bot.0 != 0 && bot.1 != 0
}

fn process_instructions(input: &str, desired: (usize, usize), part: usize) -> usize {
    let size = input.lines().count();
    let mut bots = vec![(0, 0); size]; // 231 lines in inputfile, decent enough choice
    let mut transfers = vec![(false, 0, false, 0); size];
    let mut outputs = vec![0; size];

    let re_value: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let re_transfer: Regex =
        Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)")
            .unwrap();
    // process begin situation
    let mut bots_todo = vec![];
    let mut seen_bots = HashSet::new();
    for cap in re_value.captures_iter(input) {
        let bot_index = cap[2].parse::<usize>().unwrap();
        if seen_bots.contains(&bot_index) {
            bots_todo.push(bot_index);
        } else {
            seen_bots.insert(bot_index);
        }
        let received_value = cap[1].parse::<usize>().unwrap();
        bots[bot_index] = add_value_to_tuple(bots[bot_index], received_value);
    }
    for cap in re_transfer.captures_iter(input) {
        let giver_bot_index = cap[1].parse::<usize>().unwrap();
        let target_1 = &cap[2];
        let target_1_index = cap[3].parse::<usize>().unwrap();
        let target_2 = &cap[4];
        let target_2_index = cap[5].parse::<usize>().unwrap();
        transfers[giver_bot_index] = (
            target_1 == "bot",
            target_1_index,
            target_2 == "bot",
            target_2_index,
        );
    }
    while !bots_todo.is_empty() {
        let giver_bot_index = bots_todo.pop().unwrap();
        let (target_1_bot, target_1_index, target_2_bot, target_2_index) =
            transfers[giver_bot_index];
        if target_1_bot {
            bots[target_1_index] =
                add_value_to_tuple(bots[target_1_index], bots[giver_bot_index].0);
            if part == 1 && bots[target_1_index] == desired {
                return target_1_index;
            }
            bots[giver_bot_index] = remove_value_from_tuple(bots[giver_bot_index], 0);
            if is_bot_full(bots[target_1_index]) {
                bots_todo.push(target_1_index);
            }
        } else {
            outputs[target_1_index] = bots[giver_bot_index].0;
        }
        if target_2_bot {
            bots[target_2_index] =
                add_value_to_tuple(bots[target_2_index], bots[giver_bot_index].1);
            if part == 1 && bots[target_2_index] == desired {
                return target_2_index;
            }
            bots[giver_bot_index] = remove_value_from_tuple(bots[giver_bot_index], 1);
            if is_bot_full(bots[target_2_index]) {
                bots_todo.push(target_2_index);
            }
        } else {
            outputs[target_2_index] = bots[giver_bot_index].1;
        }
    }
    if part == 2 {
        return outputs[..3].iter().product();
    }
    panic!("Could not solve the problem!");
}

pub fn main() {
    let input = read_to_string("inputs/day10.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "73";
    let part_1: usize = process_instructions(&input, (17, 61), 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "3965";
    let part_2: usize = process_instructions(&input, (0, 0), 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "value 5 goes to bot 2\nbot 2 gives low to bot 1 and high to bot 0\nvalue 3 goes to bot 1\nbot 1 gives low to output 1 and high to bot 0\nbot 0 gives low to output 2 and high to output 0\nvalue 2 goes to bot 2";
        let answer: usize = process_instructions(&input, (3, 5), 1);
        assert_eq!(answer, 0);
    }
}
