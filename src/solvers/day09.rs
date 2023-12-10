use crate::Solver;
use itertools::Itertools;

pub struct Day09;

fn get_history(input: &str) -> Vec<i32> {
    input
        .split(' ')
        .filter_map(|num| num.parse::<i32>().ok())
        .collect::<Vec<_>>()
}

fn get_sequences(history: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut sequences = vec![get_steps(&history)];

    while !sequences.last().unwrap().iter().all_equal() {
        sequences.push(get_steps(sequences.last().unwrap()));
    }

    sequences
}

fn get_steps(sequence: &Vec<i32>) -> Vec<i32> {
    sequence
        .iter()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .collect()
}

impl Solver for Day09 {
    fn star_one(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let history = get_history(line);

                let add_value = get_sequences(&history)
                    .iter()
                    .rev()
                    .map(|seq| seq.last().unwrap().clone())
                    .reduce(|acc, x| acc + x)
                    .unwrap();

                history.last().unwrap() + add_value
            })
            .sum::<i32>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let history = get_history(line);

                let minus_value = get_sequences(&history)
                    .iter()
                    .rev()
                    .map(|seq| seq.first().unwrap().clone())
                    .reduce(|acc, x| x - acc)
                    .unwrap();

                history.first().unwrap() - minus_value
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_star_one() {
        let solver = Day09 {};
        assert_eq!(solver.star_one(TEST_DATA), "114");
    }

    #[test]
    fn test_star_two() {
        let solver = Day09 {};
        assert_eq!(solver.star_two(TEST_DATA), "2");
    }
}
