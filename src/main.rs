use advent_of_code_2023::{get_solver, LATEST_DAY};
use clap::{value_parser, Parser};

#[derive(Parser)]
struct Args {
    #[arg(
        help = "The day to run puzzles for. If left blank the latest day to be solved will run.",
        value_parser = value_parser!(u8).range(1..=25)
    )]
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let day = args.day.unwrap_or(LATEST_DAY);
    let padded_day = format!("{day:0>2}");

    println!("AoC 2023: Day {padded_day}");
    println!("================");
    println!();

    let Some(solver) = get_solver(day) else {
        println!("unimplemented");
        return;
    };

    let input_file_name = format!("inputs/day{padded_day}.txt");
    let Ok(input) = std::fs::read_to_string(&input_file_name) else {
        println!("cannot read {input_file_name}");
        return;
    };

    println!("Star 1: {}", solver.star_one(&input));
    println!("Star 2: {}", solver.star_two(&input));
}
