use crate::Solver;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
enum JType {
    Jokers = 1,
    Jacks = 11,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct CardHand {
    hand: Vec<u64>,
    bid: u64,
    hand_type: HandType,
}

impl CardHand {
    fn from(input: &str, j_type: JType) -> CardHand {
        let (hand, bid) = input.split_once(' ').unwrap();

        let hand = hand
            .chars()
            .map(|card| match card {
                '2'..='9' => card.to_digit(10).unwrap() as u64,
                'T' => 10,
                'J' => j_type as u64,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!("malformed input"),
            })
            .collect::<Vec<_>>();

        let bid = bid.parse::<u64>().unwrap();

        let counts = hand.iter().counts();
        let hand_type = match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if hand.contains(&1) {
                    HandType::FiveOfAKind
                } else if counts.values().contains(&4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if counts.values().contains(&3) {
                    if hand.contains(&1) {
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else if counts.get(&1) == Some(&2) {
                    HandType::FourOfAKind
                } else if counts.get(&1) == Some(&1) {
                    HandType::FullHouse
                } else {
                    HandType::TwoPair
                }
            }
            4 => {
                if hand.contains(&1) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            _ => {
                if hand.contains(&1) {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
        };

        CardHand {
            hand,
            bid,
            hand_type,
        }
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);

        if hand_type_cmp != Ordering::Equal {
            return hand_type_cmp;
        } else {
            for i in 0..5 {
                let value_cmp = self.hand[i].cmp(&other.hand[i]);
                if value_cmp != Ordering::Equal {
                    return value_cmp;
                }
            }
        }

        Ordering::Equal
    }
}

pub struct Day07;

impl Solver for Day07 {
    fn star_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| CardHand::from(line, JType::Jacks))
            .sorted()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u64 + 1))
            .sum::<u64>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| CardHand::from(line, JType::Jokers))
            .sorted()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u64 + 1))
            .sum::<u64>()
            .to_string()
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

    #[test]
    fn test_star_two() {
        let solver = Day07 {};
        assert_eq!(solver.star_two(TEST_DATA), "5905");
    }
}
