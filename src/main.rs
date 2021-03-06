use structopt::StructOpt;

mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day18;
mod day19;
mod day21;
mod day22;

#[derive(StructOpt)]
#[structopt(
    name = "aoc-2016",
    about = "Codebase for all of the 2016 Advent of Code challenges in Rust"
)]
struct Opt {
    /// Specify day to run
    #[structopt(short = "d", long = "day", default_value = "all")]
    day: String,
}

fn print_day_header(day: usize) {
    println!(
        "------------------------------------ DAY {} ------------------------------------",
        day
    );
}

fn main() {
    let args = Opt::from_args();
    let mains = [
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
        day08::main,
        day09::main,
        day10::main,
        day12::main,
        day13::main,
        day14::main,
        day15::main,
        day16::main,
        day18::main,
        day19::main,
        day21::main,
        day22::main,
    ];

    match args.day.as_str() {
        "all" => {
            for (day, main) in mains.iter().enumerate() {
                print_day_header(day + 1);
                main();
                println!();
            }
        }
        _ => {
            let day: usize = args.day.parse().unwrap();
            print_day_header(day);
            mains[day - 1]();
        }
    }
}
