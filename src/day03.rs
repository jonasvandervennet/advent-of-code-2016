use crate::util::{print_part_1, print_part_2};
use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn check_single_triangle(sides: &Vec<usize>) -> bool {
    for order in sides.iter().permutations(3) {
        if order[0] + order[1] <= *order[2] {
            // invalid triangle
            return false;
        }
    }
    true
}

fn verify_triangles(input: &str, part: usize) -> usize {
    let mut count_valid = 0;
    let mut memory = vec![vec![], vec![], vec![]];
    let re: Regex = Regex::new(r"(\d{1,})\s+(\d{1,})\s+(\d{1,})").unwrap();
    for cap in re.captures_iter(input) {
        let values = vec![
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap(),
            cap[3].parse::<usize>().unwrap(),
        ];
        if part == 1 {
            if check_single_triangle(&values) {
                count_valid += 1;
            }
        } else {
            for i in 0..3 {
                memory[i].push(values[i]);
            }
            let size = memory[0].iter().count();
            if size == 3 {
                for i in 0..3 {
                    if check_single_triangle(&memory[i]) {
                        count_valid += 1;
                    }
                }
                memory = vec![vec![], vec![], vec![]]; // reset
            }
        }
    }
    count_valid
}

pub fn main() {
    let input = read_to_string("inputs/day03.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "1032";
    let part_1: usize = verify_triangles(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "1838";
    let part_2: usize = verify_triangles(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "5 10 25";
        let answer: usize = verify_triangles(&input, 1);
        assert_eq!(answer, 0);
    }
    #[test]
    fn test_example_2() {
        let input = "5 10 13";
        let answer: usize = verify_triangles(&input, 1);
        assert_eq!(answer, 1);
    }
}
