# advent_of_code_2023

Advent of Code 2023 solutions in Rust.

I did the first 11 days of the 2022 event a couple months ago as a challenge
from an instructor. Now that the new puzzles are arriving, I feel like this
is a good way to continue learning Rust. Plus I just had fun doing it, so
there's nothing to lose.

## Project Structure

Stole the `Solver` trait and project layout from my 2022 project. It worked
well enough for me. Added the [clap](https://crates.io/crates/clap) crate
to do some command line argument parsing. This allows me to pass the day
number as an argument and only run the puzzles from that specific day. It
also leaves open the option to easily add something like a `--timed` flag
to time the solutions later.

## Day 1

Solved part one in about thirty seconds. But wow, either my brain is just tired
at this hour or I’m lacking in skill, but part two is harder than any other
year has been on the first day. Anyway, I managed to solve it, but I absolutely
hate it, and will definitely be coming back to try to clean this one up.
