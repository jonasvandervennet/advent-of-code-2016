use crate::util::{print_part_1, print_part_2};
use std::collections::VecDeque;
use std::time::Instant;

fn play_round(elves: &mut VecDeque<usize>, part: usize) {
    if elves.len() % 2 == 0 {
        let old_elves = elves.to_owned();
        elves.clear();
        for (i, &elf) in old_elves.iter().enumerate() {
            if i % 2 == 0 {
                elves.push_back(elf)
            }
        }
    } else {
        // play the game up until the last elf
        // and start a new round with that elf as the starter
        let last = elves.pop_back().unwrap();
        play_round(elves, part);
        elves.push_front(last);
    }
}

fn play_white_elephant(num_elves: usize, part: usize) -> usize {
    if part == 1 {
        let mut elves = VecDeque::new();
        for i in 1..=num_elves {
            elves.push_back(i);
        }

        while elves.len() > 1 {
            play_round(&mut elves, part);
        }
        elves[0]
    } else {
        // for num_elves a power of 3, the last elf wil win
        // between two powers of three: e.g. x and 3*x,
        let mut i = 1;
        while 3 * i < num_elves {
            i *= 3;
        }
        num_elves - i
    }
}

pub fn main() {
    let input = 3005290;

    // PART 1
    let start = Instant::now();
    let known_answer = "1816277";
    let part_1: usize = play_white_elephant(input, 1);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "1410967";
    let part_2: usize = play_white_elephant(input, 2);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = 5;
        let answer: usize = play_white_elephant(input, 1);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_example_2() {
        let input = 6;
        let answer: usize = play_white_elephant(input, 1);
        assert_eq!(answer, 5);
    }

    #[test]
    fn test_example_3() {
        let input = 8;
        let answer: usize = play_white_elephant(input, 1);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_example_2_1() {
        let input = 5;
        let answer: usize = play_white_elephant(input, 2);
        assert_eq!(answer, 2);
    }
}
