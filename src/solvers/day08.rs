use crate::Solver;
use num::Integer;
use std::collections::HashMap;

pub struct Day08;

impl Solver for Day08 {
    fn star_one(&self, input: &str) -> String {
        let (directions, map) = input.split_once("\n\n").unwrap();

        let mut route_map = HashMap::new();

        for line in map.lines() {
            let line = line.replace([' ', '('], "").replace(')', "");
            let (position, destinations) = line.split_once('=').unwrap();
            let (left, right) = destinations.split_once(',').unwrap();
            route_map.insert(position.to_string(), (left.to_string(), right.to_string()));
        }

        let mut current_position = "AAA".to_string();
        for (step, direction) in directions.chars().cycle().enumerate() {
            current_position = match direction {
                'L' => route_map[&current_position].0.to_string(),
                'R' => route_map[&current_position].1.to_string(),
                _ => unreachable!(),
            };

            if current_position == "ZZZ" {
                return (step + 1).to_string();
            }
        }

        unreachable!()
    }

    fn star_two(&self, input: &str) -> String {
        let (directions, map) = input.split_once("\n\n").unwrap();

        let mut route_map = HashMap::new();

        for line in map.lines() {
            let line = line.replace([' ', '('], "").replace(')', "");
            let (position, destinations) = line.split_once('=').unwrap();
            let (left, right) = destinations.split_once(',').unwrap();
            route_map.insert(position.to_string(), (left.to_string(), right.to_string()));
        }

        let positions = route_map
            .keys()
            .filter(|pos| pos.ends_with('A'))
            .collect::<Vec<_>>();

        let steps = positions
            .iter()
            .filter(|pos| pos.ends_with('A'))
            .map(|pos| {
                let mut current_position = pos.to_string();
                for (step, direction) in directions.chars().cycle().enumerate() {
                    current_position = match direction {
                        'L' => route_map[&current_position].0.to_string(),
                        'R' => route_map[&current_position].1.to_string(),
                        _ => unreachable!(),
                    };

                    if current_position.ends_with('Z') {
                        return step + 1;
                    }
                }
                unreachable!()
            })
            .collect::<Vec<_>>();

        steps
            .into_iter()
            .reduce(|acc, steps| acc.lcm(&steps))
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        let solver = Day08 {};
        assert_eq!(
            solver.star_one(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"
            ),
            "2"
        );

        assert_eq!(
            solver.star_one(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"
            ),
            "6"
        );
    }

    #[test]
    fn test_star_two() {
        let solver = Day08 {};
        assert_eq!(
            solver.star_two(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
            ),
            "6"
        );
    }
}
