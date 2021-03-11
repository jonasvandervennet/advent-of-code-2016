use structopt::StructOpt;

mod util;

mod day01;

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
    let mains = [day01::main];

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
