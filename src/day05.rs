use crate::util::{print_part_1, print_part_2};
use md5::{Digest, Md5};
use std::time::Instant;

fn crack_password(key: &str, part: usize) -> String {
    let mut suffix = 0;
    let mut password = vec!['0'; 8];
    let mut num_deciphered = 0; // counter for part 1
    let mut positions_todo = vec![true; 8]; // bookkeeping for part 2
    let mut hasher = Md5::new();
    loop {
        hasher.update(format!("{}{}", key, suffix));
        let finalized = hasher.finalize_reset(); // array of bytes

        // want to have hexadecimal with five leading zeros => 3 bytes should be extracted
        // but part 2 requires a seventh digit, so increase to 4 bytes
        let result = &finalized[..4];
        if result[0] == 0 && result[1] == 0 && result[2] < 16 {
            if part == 1 {
                password[num_deciphered] = format!("{:x}", result[2]).chars().nth(0).unwrap();
                num_deciphered += 1;
                if num_deciphered == 8 {
                    break;
                }
            } else {
                let position = result[2] as usize;
                if position < 8 && positions_todo[position] {
                    // if result is fully 0 in top bits, format! will not generate the 0
                    let value = if result[3] < 16 {
                        '0'
                    } else {
                        format!("{:x}", result[3]).chars().nth(0).unwrap()
                    };
                    password[position] = value;
                    positions_todo[position] = false;
                    if !positions_todo.iter().any(|&x| x) {
                        break;
                    }
                }
            }
        }
        suffix += 1;
    }
    password.into_iter().collect()
}

pub fn main() {
    let input = "ffykfhsq";
    // PART 1
    let start = Instant::now();
    let known_answer = "c6697b55";
    let part_1: String = crack_password(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "8c35d1ab";
    let part_2: String = crack_password(&input, 2);
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
//         let answer: String = crack_password(&input, 1);
//         assert_eq!(answer, "18f47a30");
//     }

//     #[test]
//     fn test_example_2() {
//         let input = "abc";
//         let answer: String = crack_password(&input, 2);
//         assert_eq!(answer, "05ace8e3");
//     }
// }
