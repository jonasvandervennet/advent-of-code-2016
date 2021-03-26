use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

struct Disc {
    index: usize,
    num_positions: usize,
    starting_position: usize,
}

fn time_button_press(input: &str, part: usize) -> usize {
    let mut discs = vec![];
    let re: Regex =
        Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).")
            .unwrap();
    for cap in re.captures_iter(input) {
        discs.push(Disc {
            index: cap[1].parse().unwrap(),
            num_positions: cap[2].parse().unwrap(),
            starting_position: cap[3].parse().unwrap(),
        });
    }
    if part == 2 {
        discs.push(Disc {
            index: discs.len() + 1,
            num_positions: 11,
            starting_position: 0,
        });
    }
    // sort in descending order of modulus
    discs.sort_by(|a, b| b.num_positions.cmp(&a.num_positions));

    // GOAL: starting_pos + index + t == 0 (mod num_positions)
    // Chinese remainder theorem:
    // update t by incrementing the product of the
    // higest relevant moduli until consensus is reached for all of them
    let mut num_relevant_moduli = 1;
    let mut increment = discs[0].num_positions;
    let mut t = increment - (discs[0].starting_position + discs[0].index) % increment;
    while num_relevant_moduli != discs.len() {
        let disc = &discs[num_relevant_moduli];
        if (disc.starting_position + disc.index + t) % disc.num_positions != 0 {
            // there is a check failing, so increment t
            t += increment;
        } else {
            // all relevant moduli are OK
            increment *= disc.num_positions;
            num_relevant_moduli += 1;
        }
    }
    t
}

pub fn main() {
    let input = read_to_string("inputs/day15.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "148737";
    let part_1: usize = time_button_press(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "2353212";
    let part_2: usize = time_button_press(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "Disc #1 has 5 positions; at time=0, it is at position 4.\nDisc #2 has 2 positions; at time=0, it is at position 1.";
        let answer: usize = time_button_press(&input, 1);
        assert_eq!(answer, 5);
    }
}
