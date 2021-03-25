use crate::util::{print_part_1, print_part_2};
use fancy_regex::Regex;
use md5::{Digest, Md5};
use std::collections::HashSet;
use std::time::Instant;

fn generate_keys(salt: &str, amount: usize, spacing: usize, part: usize) -> usize {
    let mut suffix = 0;
    let mut hasher = Md5::new();
    let re_three: Regex = Regex::new(r"([a-z]|[0-9])\1{2}").unwrap();
    let re_five: Regex = Regex::new(r"([a-z]|[0-9])\1{4}").unwrap();
    let mut threes = HashSet::new();
    let mut num_valid_keys = 0;
    loop {
        hasher.update(format!("{}{}", salt, suffix));
        let hash = if part == 1 {
            hasher.finalize_reset()
        } else {
            let mut current_hash = hasher.finalize_reset();
            for _ in 0..2016 {
                hasher.update(format!("{:x}", current_hash));
                current_hash = hasher.finalize_reset();
            }
            current_hash
        };
        let lowercase_hex_hash = format!("{:x}", hash);

        if re_five.is_match(&lowercase_hex_hash).unwrap() {
            let result = re_three.captures(&lowercase_hex_hash);
            let captures = result
                .expect("Error running regex")
                .expect("No match found");
            let kind = captures
                .get(1)
                .expect("No group")
                .as_str()
                .chars()
                .nth(0)
                .unwrap();
            let mut to_remove: Vec<(usize, char)> = vec![];
            for &(start, three_kind) in threes.iter() {
                if kind == three_kind && start + spacing >= suffix {
                    to_remove.push((start, three_kind));
                }
            }
            to_remove.sort_by(|&a, &b| a.0.partial_cmp(&b.0).unwrap());
            for &(start, three_kind) in to_remove.iter() {
                num_valid_keys += 1;
                if num_valid_keys == amount {
                    return start;
                }
                threes.remove(&(start, three_kind));
            }
        }
        // don't gate the 3-match if there was a 5-match, 5 counts as a new 3 as well!
        if re_three.is_match(&lowercase_hex_hash).unwrap() {
            let result = re_three.captures(&lowercase_hex_hash);
            let captures = result
                .expect("Error running regex")
                .expect("No match found");
            let kind = captures
                .get(1)
                .expect("No group")
                .as_str()
                .chars()
                .nth(0)
                .unwrap();
            threes.insert((suffix, kind));
        }

        suffix += 1;
        threes = threes
            .into_iter()
            .filter(|&(start, _)| start + spacing >= suffix)
            .collect();
    }
}

pub fn main() {
    let input = "jlmsuwbz";

    // PART 1
    let start = Instant::now();
    let known_answer = "35186";
    let part_1: usize = generate_keys(&input, 64, 1000, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "22429";
    let part_2: usize = generate_keys(&input, 64, 1000, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

// Test cases commented out because they take a very long time
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_example_1() {
//         let input = "abc";
//         let answer: usize = generate_keys(&input, 64, 1000, 1);
//         assert_eq!(answer, 22728);
//     }
//     #[test]
//     fn test_example_2() {
//         let input = "abc";
//         let answer: usize = generate_keys(&input, 64, 1000, 2);
//         assert_eq!(answer, 22551);
//     }
// }
