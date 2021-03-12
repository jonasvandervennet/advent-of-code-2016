use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn move_from_number(cur_number: char, movement: &char, part: usize) -> char {
    // absolute value of travelled distance in 2 dimensions
    if part == 1 {
        let cur_number = cur_number as u8 - 48; // convert to digit value
        let new = match movement {
            'U' => match cur_number {
                1 | 2 | 3 => cur_number,
                _ => cur_number - 3,
            },
            'L' => match cur_number {
                1 | 4 | 7 => cur_number,
                _ => cur_number - 1,
            },
            'D' => match cur_number {
                7 | 8 | 9 => cur_number,
                _ => cur_number + 3,
            },
            'R' => match cur_number {
                3 | 6 | 9 => cur_number,
                _ => cur_number + 1,
            },
            _ => unreachable!(),
        };
        (new + 48) as char // convert back to character
    } else {
        match movement {
            'U' => match cur_number {
                '1' | '2' | '4' | '5' | '9' => cur_number,
                '3' => '1',
                '6' => '2',
                '7' => '3',
                '8' => '4',
                'A' => '6',
                'B' => '7',
                'C' => '8',
                'D' => 'B',
                _ => unreachable!(),
            },
            'L' => match cur_number {
                '1' | '2' | '5' | 'A' | 'D' => cur_number,
                '3' => '2',
                '4' => '3',
                '6' => '5',
                '7' => '6',
                '8' => '7',
                '9' => '8',
                'B' => 'A',
                'C' => 'B',
                _ => unreachable!(),
            },
            'D' => match cur_number {
                '5' | 'A' | 'D' | 'C' | '9' => cur_number,
                '1' => '3',
                '2' => '6',
                '3' => '7',
                '4' => '8',
                '6' => 'A',
                '7' => 'B',
                '8' => 'C',
                'B' => 'D',
                _ => unreachable!(),
            },
            'R' => match cur_number {
                '1' | '4' | '9' | 'C' | 'D' => cur_number,
                '2' => '3',
                '3' => '4',
                '5' => '6',
                '6' => '7',
                '7' => '8',
                '8' => '9',
                'A' => 'B',
                'B' => 'C',
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

fn follow_keypad(input: &str, part: usize) -> String {
    let mut current = '5';
    let mut numbers_pressed = vec![];
    for line in input.lines() {
        for c in line.chars() {
            current = move_from_number(current, &c, part);
        }
        numbers_pressed.push(current);
    }

    // let mut code = 0;
    // let size = numbers_pressed.iter().count();
    // for (i, n) in numbers_pressed.iter().enumerate() {
    //     code += n * usize::pow(10, (size - i - 1) as u32);
    // }
    // code
    numbers_pressed.into_iter().collect()
}

pub fn main() {
    let input = read_to_string("inputs/day02.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "18843";
    let part_1: String = follow_keypad(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "67BB9";
    let part_2: String = follow_keypad(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let answer: String = follow_keypad(&input, 1);
        assert_eq!(answer, "1985");
    }

    #[test]
    fn test_example_2() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let answer: String = follow_keypad(&input, 2);
        assert_eq!(answer, "5DB3");
    }
}
