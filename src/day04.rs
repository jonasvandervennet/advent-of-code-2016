use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn solve(input: &str, part: usize) -> usize {
    // split capture into name/ sector ID/checksum
    let re: Regex = Regex::new(r"([\w-]+)-(\d+)\[(\w+)\]").unwrap();

    let mut valid_sector_ids = 0;
    for cap in re.captures_iter(input) {
        let name = &cap[1];
        let sector_id = cap[2].parse::<usize>().unwrap();
        let given_checksum = &cap[3];

        if part == 1 {
            let mut char_occurences = HashMap::new();
            for c in name.chars() {
                if c == '-' {
                    continue;
                }
                char_occurences
                    .entry(c)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            let mut count_vec: Vec<_> = char_occurences.iter().collect();
            count_vec.sort_by(|a, b| a.0.cmp(b.0)); // order alphabetically on characters
            count_vec.sort_by(|a, b| b.1.cmp(a.1)); // stable sort on key values
            let calculated_checksum: String = count_vec[..5].iter().map(|&(k, _)| k).collect();
            
            if calculated_checksum == given_checksum {
                valid_sector_ids += sector_id;
            }
        } else {
            let increment = sector_id % 26; // shift cipher on alphabet only
            let decrypted_name: String = name
                .chars()
                .map(|c| {
                    if c == '-' {
                        ' '
                    } else {
                        let mut number_value = c as usize + increment;
                        if number_value > 122 {
                            number_value -= (122 - 97) + 1
                        }
                        number_value as u8 as char
                    }
                })
                .collect();
            // println!("{} -> {}", sector_id, decrypted_name);
            if decrypted_name == "northpole object storage"{
                return sector_id;
            }
        }
    }
    valid_sector_ids
}

pub fn main() {
    let input = read_to_string("inputs/day04.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "245102";
    let part_1: usize = solve(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "324";
    let part_2: usize = solve(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let answer: usize = solve(&input, 1);
        assert_eq!(answer, 123);
    }

    #[test]
    fn test_example_2() {
        let input = "a-b-c-d-e-f-g-h-987[abcde]";
        let answer: usize = solve(&input, 1);
        assert_eq!(answer, 987);
    }

    #[test]
    fn test_example_3() {
        let input = "not-a-real-room-404[oarel]";
        let answer: usize = solve(&input, 1);
        assert_eq!(answer, 404);
    }

    #[test]
    fn test_example_4() {
        let input = "totally-real-room-200[decoy]";
        let answer: usize = solve(&input, 1);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_2_1() {
        let input = "qzmt-zixmtkozy-ivhz-343[dummy]";
        let answer: usize = solve(&input, 2);
        assert_eq!(answer, 0);
    }
}
