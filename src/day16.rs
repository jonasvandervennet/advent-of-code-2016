use crate::util::{print_part_1, print_part_2};
use std::time::Instant;

fn checksum(data: Vec<bool>) -> Vec<bool> {
    let mut data = data;
    loop {
        let mut checksum_bits = vec![];
        let mut i = 0;
        while i < data.len() - 1 {
            checksum_bits.push(data[i] == data[i + 1]);
            i += 2;
        }
        if checksum_bits.len() % 2 == 1 {
            return checksum_bits;
        }
        data = checksum_bits;
    }
}

fn iteration(a: &mut Vec<bool>) {
    let mut b = a.to_owned();
    b.reverse();
    b = b.iter().map(|i| !i).collect();
    a.push(false);
    a.extend(b);
}

fn fill_disk_checksum(input: &str, disk_size: usize) -> String {
    let mut data: Vec<bool> = input
        .chars()
        .map(|c| {
            if c == '1' {
                true
            } else if c == '0' {
                false
            } else {
                panic!("bad input")
            }
        })
        .collect();
    while data.len() < disk_size {
        iteration(&mut data);
    }
    data.resize(disk_size, false); // discard extra entries exceeding disk size
    let checksum_bits = checksum(data);
    checksum_bits
        .iter()
        .map(|&c| if c { '1' } else { '0' })
        .collect()
}

pub fn main() {
    let input = "00101000101111010";

    // PART 1
    let start = Instant::now();
    let known_answer = "10010100110011100";
    let part_1: String = fill_disk_checksum(&input, 272);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "01100100101101100";
    let part_2: String = fill_disk_checksum(&input, 35651584);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "10000";
        let answer: String = fill_disk_checksum(&input, 20);
        assert_eq!(answer, "01100");
    }
}
