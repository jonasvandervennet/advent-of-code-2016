use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn process_instructions(input: &str, part: usize) -> usize {
    let compressed: Vec<char> = input.chars().collect();
    let mut decompressed_length = 0;

    let re: Regex = Regex::new(r"(\d+)x(\d+)").unwrap();
    let mut i = 0;
    while i < compressed.len() {
        let current = compressed[i];
        i += 1;
        if current == '(' {
            // start of marker
            let mut marker_content = vec![];
            loop {
                let next = compressed[i];
                i += 1;
                if next == ')' {
                    break;
                }
                marker_content.push(next);
            }
            for cap in re.captures_iter(&marker_content.iter().collect::<String>()) {
                let repeat_length = cap[1].parse::<usize>().unwrap();
                let repeat_amount = cap[2].parse::<usize>().unwrap();
                let repeat_contains_bracket = compressed[i..i + repeat_length].contains(&'(');
                if part == 2 && repeat_contains_bracket {
                    // recursively expand sequence if it still contains markers
                    let repeat_sequence: String = compressed[i..i + repeat_length].iter().collect();
                    decompressed_length +=
                        process_instructions(&repeat_sequence, part) * repeat_amount;
                } else {
                    decompressed_length += repeat_amount * repeat_length;
                }
                i += repeat_length;
            }
        } else {
            decompressed_length += 1;
        }
    }
    decompressed_length
}

pub fn main() {
    let input = read_to_string("inputs/day09.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "115118";
    let part_1: usize = process_instructions(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "11107527530";
    let part_2: usize = process_instructions(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "ADVENT";
        let answer: usize = process_instructions(&input, 1);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_example_2() {
        let input = "A(1x5)BC";
        let answer: usize = process_instructions(&input, 1);
        assert_eq!(answer, 7);
    }

    #[test]
    fn test_example_3() {
        let input = "(3x3)XYZ";
        let answer: usize = process_instructions(&input, 1);
        assert_eq!(answer, 9);
    }

    #[test]
    fn test_example_4() {
        let input = "A(2x2)BCD(2x2)EFG";
        let answer: usize = process_instructions(&input, 1);
        assert_eq!(answer, 11);
    }

    #[test]
    fn test_example_5() {
        let input = "(6x1)(1x3)A";
        let answer: usize = process_instructions(&input, 1);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_example_6() {
        let input = "X(8x2)(3x3)ABCY";
        let answer: usize = process_instructions(&input, 1);
        assert_eq!(answer, 18);
    }

    #[test]
    fn test_example_2_1() {
        let input = "(3x3)XYZ";
        let answer: usize = process_instructions(&input, 2);
        assert_eq!(answer, 9);
    }

    #[test]
    fn test_example_2_2() {
        let input = "X(8x2)(3x3)ABCY";
        let answer: usize = process_instructions(&input, 2);
        assert_eq!(answer, 20);
    }

    #[test]
    fn test_example_2_3() {
        let input = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        let answer: usize = process_instructions(&input, 2);
        assert_eq!(answer, 241920);
    }

    #[test]
    fn test_example_2_4() {
        let input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        let answer: usize = process_instructions(&input, 2);
        assert_eq!(answer, 445);
    }
}
