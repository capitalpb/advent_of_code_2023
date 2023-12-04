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

## Day 3

Another day of the 2023 Advent of Code, and another day where I hate looking
at my code. This year just seems like it is starting off a lot more complex
than I remember in previous years. This one was a little tricky, but I got
there without any major setbacks. Another one I am excited to come back to and
clean up, but this first pass is all about getting a solution, and this one
works.

## Day 4

I enjoyed this one. It was a nice simple break after Days 1 and 3; the type of
basic puzzle I expect from the first few days of Advent of Code. Pretty simple
logic in this one, I don't think I would change too much. I'm sure I'll find a
way to clean up how it's written a bit, but I'm happy with this one today.
