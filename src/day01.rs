use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn simplify_walked_dimensions(dist_per_direction: &Vec<isize>) -> (isize, isize) {
    // absolute value of travelled distance in 2 dimensions
    let vertical_distance = dist_per_direction[0] - dist_per_direction[2];
    let horizontal_distance = dist_per_direction[1] - dist_per_direction[3];
    (vertical_distance, horizontal_distance)
}

fn walk_along_input(input: &str, part: usize) -> isize {
    let mut seen_locations: HashSet<(isize, isize)> = HashSet::new();
    let mut travelled_in_direction: Vec<isize> = vec![0, 0, 0, 0]; // up, right, down, left
    let mut direction_index: usize = 0;
    let mut visted_twice: bool = false;

    let re: Regex = Regex::new(r"([LR])(\d{1,})").unwrap();
    'isns: for cap in re.captures_iter(input) {
        let dir_change = &cap[1].parse::<char>().unwrap();
        let mut val = cap[2].parse::<usize>().unwrap();

        match dir_change {
            'L' => {
                direction_index = if direction_index > 0 {
                    direction_index - 1
                } else {
                    3
                };
            }
            'R' => {
                direction_index = if direction_index < 3 {
                    direction_index + 1
                } else {
                    0
                };
            }
            _ => unreachable!(),
        }
        while val > 0 {
            travelled_in_direction[direction_index] += 1;
            if part == 2 {
                let reduced = simplify_walked_dimensions(&travelled_in_direction);
                if seen_locations.contains(&reduced) {
                    visted_twice = true;
                    break 'isns; // stop once location has been reached twice
                }
                seen_locations.insert(reduced);
            }
            val -= 1;
        }
    }
    if part == 2 && visted_twice == false {
        return 0;
    }
    let walked = simplify_walked_dimensions(&travelled_in_direction);
    (walked.0 + walked.1).abs() // total manhattan distance
}

pub fn main() {
    let input = read_to_string("inputs/day01.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "353";
    let part_1: isize = walk_along_input(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "152";
    let part_2: isize = walk_along_input(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "R2, L3";
        let answer: isize = walk_along_input(&input, 1);
        assert_eq!(answer, 5);
    }

    #[test]
    fn test_example_2() {
        let input = "R2, R2, R2";
        let answer: isize = walk_along_input(&input, 1);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_example_3() {
        let input = "R5, L5, R5, R3";
        let answer: isize = walk_along_input(&input, 1);
        assert_eq!(answer, 12);
    }

    #[test]
    fn test_example_2_1() {
        let input = "R8, R4, R4, R8";
        let answer: isize = walk_along_input(&input, 2);
        assert_eq!(answer, 4);
    }
}
