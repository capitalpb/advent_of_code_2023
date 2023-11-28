use advent_of_code_2023::{init_solver, LATEST_DAY};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(help = "An optional day to run puzzles for. Runs all days if unspecified.")]
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let start = args.day.unwrap_or(1);
    let end = args.day.unwrap_or(LATEST_DAY);

    println!("Advent of Code 2023");
    println!("===================");

    for day in start..=end {
        println!();

        let Some(solver) = init_solver(day) else {
            println!("Day {day:0>2}: not implemented");
            continue;
        };

        let Ok(input) = std::fs::read_to_string(format!("inputs/day{day:0>2}.txt")) else {
            println!("Day {day:0>2}: cannot read input file");
            continue;
        };

        println!("Day {day:0>2} Star 1: {}", solver.star_one(&input));
        println!("Day {day:0>2} Star 2: {}", solver.star_two(&input));
    }
}
