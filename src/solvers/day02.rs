use crate::Solver;

#[derive(Debug)]
struct Hand {
    blue: usize,
    green: usize,
    red: usize,
}

impl Hand {
    fn from(input: &str) -> Hand {
        let mut hand = Hand {
            blue: 0,
            green: 0,
            red: 0,
        };

        for color in input.split(", ") {
            let color = color.split_once(' ').unwrap();
            match color.1 {
                "blue" => hand.blue = color.0.parse::<usize>().unwrap(),
                "green" => hand.green = color.0.parse::<usize>().unwrap(),
                "red" => hand.red = color.0.parse::<usize>().unwrap(),
                _ => unreachable!("malformed input"),
            }
        }

        hand
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    hands: Vec<Hand>,
}

impl Game {
    fn from(input: &str) -> Game {
        let (id, hands) = input.split_once(": ").unwrap();
        let id = id.split_once(" ").unwrap().1.parse::<usize>().unwrap();
        let hands = hands.split("; ").map(Hand::from).collect();
        Game { id, hands }
    }
}

pub struct Day02;

impl Solver for Day02 {
    fn star_one(&self, input: &str) -> String {
        input
            .lines()
            .map(Game::from)
            .filter(|game| {
                game.hands
                    .iter()
                    .all(|hand| hand.blue <= 14 && hand.green <= 13 && hand.red <= 12)
            })
            .map(|game| game.id)
            .sum::<usize>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        input
            .lines()
            .map(Game::from)
            .map(|game| {
                let max_blue = game.hands.iter().map(|hand| hand.blue).max().unwrap();
                let max_green = game.hands.iter().map(|hand| hand.green).max().unwrap();
                let max_red = game.hands.iter().map(|hand| hand.red).max().unwrap();

                max_blue * max_green * max_red
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_star_one() {
        let solver = Day02 {};
        assert_eq!(solver.star_one(TEST_DATA), "8");
    }

    #[test]
    fn test_star_two() {
        let solver = Day02 {};
        assert_eq!(solver.star_two(TEST_DATA), "2286");
    }
}
