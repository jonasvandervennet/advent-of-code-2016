use std::time::Duration;

fn print_answer(part_nr: usize, answer: &str, known_answer: &str, duration: Duration) {
    if answer != known_answer {
        print!("INCORRECT || ")
    }
    println!("PART {}: {}", part_nr, answer);
    println!("\t[{:?}]", duration);
}

pub fn print_part_1(answer: &str, known_answer: &str, duration: Duration) {
    print_answer(1, answer, known_answer, duration);
}
pub fn print_part_2(answer: &str, known_answer: &str, duration: Duration) {
    print_answer(2, answer, known_answer, duration);
}
