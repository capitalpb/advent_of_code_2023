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
        let mut result = 0;

        for line in input.lines() {
            let mut first = None;
            let mut last = None;

            while first.is_none() {
                for index in 0..line.len() {
                    let line_slice = &line[index..];
                    if line_slice.starts_with("one") || line_slice.starts_with('1') {
                        first = Some(1);
                    } else if line_slice.starts_with("two") || line_slice.starts_with('2') {
                        first = Some(2);
                    } else if line_slice.starts_with("three") || line_slice.starts_with('3') {
                        first = Some(3);
                    } else if line_slice.starts_with("four") || line_slice.starts_with('4') {
                        first = Some(4);
                    } else if line_slice.starts_with("five") || line_slice.starts_with('5') {
                        first = Some(5);
                    } else if line_slice.starts_with("six") || line_slice.starts_with('6') {
                        first = Some(6);
                    } else if line_slice.starts_with("seven") || line_slice.starts_with('7') {
                        first = Some(7);
                    } else if line_slice.starts_with("eight") || line_slice.starts_with('8') {
                        first = Some(8);
                    } else if line_slice.starts_with("nine") || line_slice.starts_with('9') {
                        first = Some(9);
                    }

                    if first.is_some() {
                        break;
                    }
                }
            }

            while last.is_none() {
                for index in (0..line.len()).rev() {
                    let line_slice = &line[index..];
                    if line_slice.starts_with("one") || line_slice.starts_with('1') {
                        last = Some(1);
                    } else if line_slice.starts_with("two") || line_slice.starts_with('2') {
                        last = Some(2);
                    } else if line_slice.starts_with("three") || line_slice.starts_with('3') {
                        last = Some(3);
                    } else if line_slice.starts_with("four") || line_slice.starts_with('4') {
                        last = Some(4);
                    } else if line_slice.starts_with("five") || line_slice.starts_with('5') {
                        last = Some(5);
                    } else if line_slice.starts_with("six") || line_slice.starts_with('6') {
                        last = Some(6);
                    } else if line_slice.starts_with("seven") || line_slice.starts_with('7') {
                        last = Some(7);
                    } else if line_slice.starts_with("eight") || line_slice.starts_with('8') {
                        last = Some(8);
                    } else if line_slice.starts_with("nine") || line_slice.starts_with('9') {
                        last = Some(9);
                    }

                    if last.is_some() {
                        break;
                    }
                }
            }

            result += format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<i32>()
                .unwrap();
        }

        result.to_string()
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
