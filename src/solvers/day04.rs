use crate::Solver;
use std::collections::HashSet;

struct Scratchcard {
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
}

impl Scratchcard {
    fn from(input: &str) -> Scratchcard {
        let (_, numbers) = input.split_once(':').unwrap();
        let (winning_numbers, player_numbers) = numbers.split_once('|').unwrap();
        let winning_numbers = winning_numbers
            .split_ascii_whitespace()
            .filter_map(|number| number.parse::<u32>().ok())
            .collect::<HashSet<_>>();
        let player_numbers = player_numbers
            .split_ascii_whitespace()
            .filter_map(|number| number.parse::<u32>().ok())
            .collect::<HashSet<_>>();

        Scratchcard {
            winning_numbers,
            player_numbers,
        }
    }

    fn matches(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.player_numbers)
            .count() as u32
    }
}

pub struct Day04;

impl Solver for Day04 {
    fn star_one(&self, input: &str) -> String {
        input
            .lines()
            .map(Scratchcard::from)
            .map(|card| {
                let matches = card.matches();
                if matches == 0 {
                    0
                } else {
                    2u32.pow(matches - 1)
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let cards: Vec<Scratchcard> = input.lines().map(Scratchcard::from).collect();
        let mut card_counts = vec![1usize; cards.len()];

        for card_number in 0..cards.len() {
            let matches = cards[card_number].matches();

            if matches == 0 {
                continue;
            }

            for i in 1..=matches {
                card_counts[card_number + i as usize] += card_counts[card_number];
            }
        }

        card_counts.iter().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_star_one() {
        let solver = Day04 {};
        assert_eq!(solver.star_one(TEST_DATA), "13");
    }

    #[test]
    fn test_star_two() {
        let solver = Day04 {};
        assert_eq!(solver.star_two(TEST_DATA), "30");
    }
}
