use crate::util::{print_part_1, print_part_2};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

type PosCharMap = HashMap<usize, HashMap<char, usize>>;

fn recover_message(input: &str, part: usize) -> String {
    let mut position_char_occurences: PosCharMap = HashMap::new();
    for word in input.lines() {
        for (pos, c) in word.chars().enumerate() {
            if !position_char_occurences.contains_key(&pos) {
                position_char_occurences.insert(pos, HashMap::new());
            }
            position_char_occurences.entry(pos).and_modify(|e| {
                e.entry(c).and_modify(|f| *f += 1).or_insert(1);
            });
        }
    }
    let mut message = vec!['0'; position_char_occurences.iter().count()];
    for (&pos, char_occ) in position_char_occurences.iter() {
        // either minimize or maximize the amount of occurences, depending on the part
        message[pos] = if part == 1 {
            *char_occ.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0
        } else {
            *char_occ.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().0
        }
    }
    message.into_iter().collect()
}

pub fn main() {
    let input = read_to_string("inputs/day06.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "gyvwpxaz";
    let part_1: String = recover_message(&input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "jucfoary";
    let part_2: String = recover_message(&input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        let answer: String = recover_message(&input, 1);
        assert_eq!(answer, "easter");
    }

    #[test]
    fn test_example_2() {
        let input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        let answer: String = recover_message(&input, 2);
        assert_eq!(answer, "advent");
    }
}
