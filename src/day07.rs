use crate::util::{print_part_1, print_part_2};
use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

fn contains_abba(input: &str) -> bool {
    for i in 0..=input.chars().count() - 4 {
        let a = input.chars().nth(i).unwrap();
        let b = input.chars().nth(i + 1).unwrap();
        let c = input.chars().nth(i + 2).unwrap();
        let d = input.chars().nth(i + 3).unwrap();
        if a != b && a == d && b == c {
            return true;
        }
    }
    false
}

fn is_aba(a: char, b: char, c: char) -> bool {
    a != b && a == c
}

fn extract_aba(input: &str) -> Vec<String> {
    let mut abas = vec![];
    for i in 0..=input.chars().count() - 3 {
        let a = input.chars().nth(i).unwrap();
        let b = input.chars().nth(i + 1).unwrap();
        let c = input.chars().nth(i + 2).unwrap();
        if is_aba(a, b, c) {
            abas.push(format!("{}{}{}", a, b, c));
        }
    }
    abas
}

fn matching_aba(aba: &str, bab: &str) -> bool {
    let a_a = aba.chars().nth(0).unwrap();
    let a_b = aba.chars().nth(1).unwrap();
    let b_b = bab.chars().nth(0).unwrap();
    let b_a = bab.chars().nth(1).unwrap();
    if a_a == b_a && a_b == b_b {
        return true;
    }
    false
}

fn check_tls_support(input: &str) -> usize {
    let mut valid_counter = 0;

    let re: Regex = Regex::new(r"(\w+)").unwrap();
    'next_address: for line in input.lines() {
        let mut one_abba = false;
        let mut want_abba = true;
        for cap in re.captures_iter(line) {
            if !want_abba || !one_abba {
                if contains_abba(&cap[1]) {
                    if want_abba {
                        one_abba = true;
                    } else {
                        continue 'next_address;
                    }
                }
            }
            want_abba = !want_abba;
        }
        if one_abba {
            valid_counter += 1;
        }
    }
    valid_counter
}

fn check_ssl_support(input: &str) -> usize {
    let mut valid_counter = 0;

    let re: Regex = Regex::new(r"(\w+)").unwrap();
    'next_address: for line in input.lines() {
        let mut abas = vec![];
        let mut babs = vec![];

        let mut want_aba = true;
        for cap in re.captures_iter(line) {
            let extracted = extract_aba(&cap[1]);
            if extracted.iter().count() > 0 {
                if want_aba {
                    abas.extend(extracted);
                } else if !want_aba {
                    // BAB is also of form 'ABA' from the viewpoint of the check
                    babs.extend(extracted);
                }
            }
            want_aba = !want_aba;
        }
        for aba in abas.iter() {
            for bab in babs.iter() {
                if matching_aba(aba, bab) {
                    valid_counter += 1;
                    continue 'next_address;
                }
            }
        }
    }
    valid_counter
}

pub fn main() {
    let input = read_to_string("inputs/day07.txt").expect("Input not found..");
    // PART 1
    let start = Instant::now();
    let known_answer = "110";
    let part_1: usize = check_tls_support(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "242";
    let part_2: usize = check_ssl_support(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "abba[mnop]qrst";
        let answer: usize = check_tls_support(&input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_example_2() {
        let input = "abcd[bddb]xyyx";
        let answer: usize = check_tls_support(&input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_3() {
        let input = "aaaa[qwer]tyui";
        let answer: usize = check_tls_support(&input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_4() {
        let input = "ioxxoj[asdfgh]zxcvbn";
        let answer: usize = check_tls_support(&input);
        assert_eq!(answer, 1);
    }
    #[test]
    fn test_example_5() {
        let input = "snwnqixjgwhcrpfeun[mvseymbltdzywnw]xbogzgtddtzzadgsrin[sibgoazaxuyfaaf]tdtrrjbxjzusuvzogpa[etytgiqwoyxevcq]ifanoaaqoldczzj";
        let answer: usize = check_tls_support(&input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_2_1() {
        let input = "aba[bab]xyz";
        let answer: usize = check_ssl_support(&input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_example_2_2() {
        let input = "xyx[xyx]xyx";
        let answer: usize = check_ssl_support(&input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_example_2_3() {
        let input = "aaa[kek]eke";
        let answer: usize = check_ssl_support(&input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_example_2_4() {
        let input = "zazbz[bzb]cdb";
        let answer: usize = check_ssl_support(&input);
        assert_eq!(answer, 1);
    }
}
