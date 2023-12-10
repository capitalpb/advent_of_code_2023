use crate::Solver;
use itertools::Itertools;

#[derive(Debug)]
struct CardHand {
    hand: Vec<u64>,
    bid: u64,
}

impl CardHand {
    fn from(input: &str) -> CardHand {
        let (hand, bid) = input.split_once(' ').unwrap();

        let hand = hand
            .chars()
            .map(|card| match card {
                '2'..='9' => card.to_digit(10).unwrap() as u64,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!("malformed input"),
            })
            .collect();

        let bid = bid.parse::<u64>().unwrap();

        CardHand { hand, bid }
    }

    fn score(&self) -> u64 {
        let counts = self.hand.iter().counts();

        let hand_type_bonus: u64 = match counts.len() {
            1 => 6_000_000_000,
            2 => {
                if counts.values().contains(&4) {
                    5_000_000_000
                } else {
                    4_000_000_000
                }
            }
            3 => {
                if counts.values().contains(&3) {
                    3_000_000_000
                } else {
                    2_000_000_000
                }
            }
            4 => 1_000_000_000,
            _ => 0,
        };

        let hand_score = self
            .hand
            .iter()
            .enumerate()
            .map(|(index, value)| value << (4 * (4 - index)))
            .sum::<u64>();

        hand_type_bonus + hand_score
    }
}

pub struct Day07;

impl Solver for Day07 {
    fn star_one(&self, input: &str) -> String {
        input
            .lines()
            .map(CardHand::from)
            .sorted_by_key(|hand| hand.score())
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u64 + 1))
            .sum::<u64>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_star_one() {
        let solver = Day07 {};
        assert_eq!(solver.star_one(TEST_DATA), "6440");
    }
}
