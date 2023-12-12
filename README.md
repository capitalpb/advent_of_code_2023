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

## Day 5

Well, I can't say much about this one. The code is ugly, horribly inefficient,
and part two takes a solid half hour to run. It got the right answer though, so
that's something I suppose. I think something like `nom` to parse the input
would be much cleaner, and there's got to be a better way of going about part
two than just brute forcing through every possible seed, but hey, it works so
that's good enough for now.

## Day 6

A nice simple one today. And only a half second delay for part two instead of
half an hour. What a treat. I could probably have nicer input parsing, but that
seems to be the theme this year, so that will become a big focus of my next
round through these I'm guessing. The algorithm here to get the winning
possibilities could also probably be improved upon by figuring out what the
number of seconds for the current record is, and only looping from there until
hitting a number that doesn't win, as opposed to brute-forcing the whole loop.

## Day 7

Two days, a few failed solutions, some misread instructions, and a lot of
manually parsing output data and debugging silly tiny mistakes... but it's
finally done. I don't really wanna talk about it.

## Day 8

First part was simple enough. Second part was easy logically, but after running
the brute force solution with a parallel iterator from `rayon` and maxing out
all 12 cores of this CPU, it was still taking forever. I always get tripped up
by these ones that need fancy math, because although I was always good at math,
I've never been good at looking at these problems and figuring out what kind
of formula would apply. So I cheated and looked at other people's comments for
their solutions, and saw the least common multiple mentioned. This made sense
to me, so I implemented it and got a result almost instantly. I hate having to
look at other comments to solve these things, but I never would have came to
that conclusion myself.

The code still isn't the cleanest, and I'd love to tidy up the parsing, but
it works and I'm happy.

## Day 9

A pretty simple one today, but fun to do. I could probably clean up the parsing
code (AKA my theme for this year), and create just one single vector instead of
having the original history separated out from all of the sequences, but this
is what made sense to me on my first pass so it's how I did it.

## Day 10

Well, star one is solved. I don't love the code, but yet again, it works for
now. I don't love the use of a label to continue/break a loop, and the
`valid_steps` function is a mess that could probably be done much cleaner.

Upon looking at star 2 I don't even have the slightest idea of where to start.
I may have to come back to this one at a later date. Sigh.

## Day 11

That was a fun one. Especially after yesterday. As soon as I saw that star 1
was expanding each gap by 1, I just had a feeling that star 2 would be doing
the same calculation with a larger expansion, so I wrote my code in a way that
would make that quite simple to modify. When I saw the factor of 1,000,000 I
was scared that it was going to be one of those processor-destroying AoC
challenges where you either wait for 2 hours to get an answer, or have to come
up with a fancy mathematical way of solving things, but after changing my `i32`
distance to an `i64`, it calculated just fine and instantly. I guess only
storing the locations of galaxys and not dealing with the entire grid was good
enough to keep the performance down.
