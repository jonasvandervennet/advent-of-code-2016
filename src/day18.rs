use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn next_row(current_row: Vec<bool>) -> Vec<bool> {
    // pad row with safe tiles left and right
    let mut row = vec![false];
    row.extend(current_row);
    row.push(false);

    let mut new_row = vec![];

    for window in row.windows(3) {
        let left = window[0];
        let center = window[1];
        let right = window[2];

        let is_trap = (center && (left ^ right))
            || (left && !center && !right)
            || (!left && !center && right);
        new_row.push(is_trap);
    }
    new_row
}

fn amount_safe_tiles(start_row: &str, num_rows: usize) -> usize {
    let mut row = 1;
    let mut current_row: Vec<_> = start_row.chars().map(|c| c == '^').collect();
    let mut total_safe = 0;

    loop {
        total_safe += current_row.iter().filter(|&&x| !x).count();
        if row == num_rows {
            return total_safe;
        }
        current_row = next_row(current_row);
        row += 1;
    }
}

pub fn main() {
    let input = read_to_string("inputs/day18.txt").expect("Input not found..");

    // PART 1
    let start = Instant::now();
    let known_answer = "1956";
    let part_1: usize = amount_safe_tiles(&input, 40);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "19995121";
    let part_2: usize = amount_safe_tiles(&input, 400000);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "..^^.";
        let answer: usize = amount_safe_tiles(&input, 3);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_example_2() {
        let input = ".^^.^.^^^^";
        let answer: usize = amount_safe_tiles(&input, 10);
        assert_eq!(answer, 38);
    }
}
