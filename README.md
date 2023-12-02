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

## Day 2

Not too tricky today. Part 2 wasn't as big of a curveball as yesterday
thankfully. I don't think it's the cleanest code I've ever written, but hey -
the whole point of this is to get better at Rust, so I'll definitely be
learning as I go, and coming back at the end to clean a lot of these up. I
think for this one I'd like to look into a parsing crate like `nom` to clean
up all the spliting and unwrapping in the two `from()` methods.
