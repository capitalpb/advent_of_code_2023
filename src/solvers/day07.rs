use crate::Solver;

#[derive(Debug)]
struct CardHand {
    hand: Vec<u32>,
    bid: u32,
}

impl CardHand {
    fn from(input: &str) -> CardHand {
        let (hand, bid) = input.split_once(' ').unwrap();

        let hand = hand
            .chars()
            .map(|card| match card {
                '2'..='9' => card.to_digit(10).unwrap() as u32,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!("malformed input"),
            })
            .collect();

        let bid = bid.parse::<u32>().unwrap();

        CardHand { hand, bid }
    }
}

pub struct Day07;

impl Solver for Day07 {
    fn star_one(&self, input: &str) -> String {
        todo!()
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
