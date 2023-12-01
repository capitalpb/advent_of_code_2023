use crate::Solver;

pub struct Day01;

impl Solver for Day01 {
    fn star_one(&self, input: &str) -> String {
        let mut result = 0;

        for line in input.lines() {
            let line = line
                .chars()
                .filter(|ch| ch.is_ascii_digit())
                .collect::<Vec<_>>();
            let first = line.first().unwrap();
            let last = line.last().unwrap();
            let number = format!("{first}{last}").parse::<i32>().unwrap();
            result += number;
        }

        result.to_string()
    }

    fn star_two(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        let solver = Day01 {};
        assert_eq!(
            solver.star_one(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
            ),
            "142"
        );
    }

    #[test]
    fn test_star_two() {
        let solver = Day01 {};
        assert_eq!(
            solver.star_two(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
            ),
            "281"
        );
    }
}
