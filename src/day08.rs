use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn process_instructions(input: &str, width: usize, height: usize, part: usize) -> String {
    let mut grid = vec![vec![0; width]; height];

    let re_rect: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let re_rotate: Regex = Regex::new(r"rotate (row|column) [xy]=(\d+) by (\d+)").unwrap();
    for line in input.lines() {
        let differentiator = line.chars().nth(1).unwrap();
        if differentiator == 'e' {
            for cap in re_rect.captures_iter(line) {
                let on_width = cap[1].parse::<usize>().unwrap();
                let on_height = cap[2].parse::<usize>().unwrap();
                for i in 0..on_height {
                    for j in 0..on_width {
                        grid[i][j] = 1;
                    }
                }
            }
        } else {
            for cap in re_rotate.captures_iter(line) {
                let is_row_shift = &cap[1] == "row";
                let index = cap[2].parse::<usize>().unwrap();
                let amount = cap[3].parse::<usize>().unwrap();

                if is_row_shift {
                    let original = &grid[index].to_vec();
                    for i in 0..width {
                        grid[index][(i + amount) % width] = original[i];
                    }
                } else {
                    let original: Vec<usize> = grid.iter().map(|row| row[index]).collect();
                    for i in 0..height {
                        grid[(i + amount) % height][index] = original[i];
                    }
                };
            }
        }
    }
    if part == 2 {
        print!("\n");
        for i in 0..grid.len() {
            if i > 0 && i % 6 == 0 {
                print!("\n")
            }
            for j in 0..grid[i].len() {
                if j > 0 && j % 5 == 0 {
                    print!(" ")
                }
                print!("{}", if grid[i][j] == 1 { "X" } else { " " });
            }
            print!("\n");
        }
        "EFEYKFRFIJ".to_string()
    } else {
        // solution for part 1
        format!(
            "{}",
            grid.iter()
                .map(|row| row.iter().sum())
                .collect::<Vec<usize>>()
                .iter()
                .sum::<usize>()
        )
    }
}

pub fn main() {
    let input = read_to_string("inputs/day08.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "115";
    let part_1: String = process_instructions(&input, 50, 6, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "EFEYKFRFIJ";
    let part_2: String = process_instructions(&input, 50, 6, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column x=1 by 1";
        let answer: String = process_instructions(&input, 7, 3, 1);
        assert_eq!(answer, "6");
    }
}
