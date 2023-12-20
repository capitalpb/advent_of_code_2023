use crate::Solver;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn possible_ways_to_win(&self) -> usize {
        (0..=self.time)
            .filter(|time| time * (self.time - time) > self.distance)
            .count()
    }
}

pub struct Day06;

impl Solver for Day06 {
    fn star_one(&self, input: &str) -> String {
        let mut race_data = input
            .lines()
            .map(|line| {
                line.split_once(':')
                    .unwrap()
                    .1
                    .split_ascii_whitespace()
                    .filter_map(|number| number.parse::<u64>().ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let times = race_data.pop().unwrap();
        let distances = race_data.pop().unwrap();

        let races = distances
            .into_iter()
            .zip(times)
            .map(|(time, distance)| Race { time, distance })
            .collect::<Vec<_>>();

        races
            .iter()
            .map(|race| race.possible_ways_to_win())
            .product::<usize>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let race_data = input
            .lines()
            .map(|line| {
                line.split_once(':')
                    .unwrap()
                    .1
                    .replace(' ', "")
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let race = Race {
            time: race_data[0],
            distance: race_data[1],
        };

        race.possible_ways_to_win().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Time:      7  15   30
    Distance:  9  40  200
";

    #[test]
    fn test_star_one() {
        let solver = Day06 {};
        assert_eq!(solver.star_one(TEST_DATA), "288");
    }

    #[test]
    fn test_star_two() {
        let solver = Day06 {};
        assert_eq!(solver.star_two(TEST_DATA), "71503");
    }
}
