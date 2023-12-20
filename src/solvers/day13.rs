use crate::Solver;

pub struct Day13;

fn find_mirror_index(input: &Vec<Vec<char>>) -> Option<usize> {
    for row in 1..input.len() {
        if input[row] == input[row - 1] {
            let slice_length = std::cmp::min(row, input.len() - row);

            if input[(row - slice_length)..row]
                .iter()
                .rev()
                .eq(input[row..(row + slice_length)].iter())
            {
                return Some(row);
            }
        }
    }

    None
}

impl Solver for Day13 {
    fn star_one(&self, input: &str) -> String {
        let sections = input
            .split("\n\n")
            .map(|section| {
                section
                    .lines()
                    .map(|line| line.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut summary = 0;

        for section in sections {
            if let Some(horizontal_mirror) = find_mirror_index(&section) {
                summary += horizontal_mirror * 100;
                continue;
            }

            let columns = (0..section[0].len())
                .map(|col| section.iter().map(|row| row[col]).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            if let Some(vertical_mirror) = find_mirror_index(&columns) {
                summary += vertical_mirror;
            }
        }

        summary.to_string()
    }

    fn star_two(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    #[test]
    fn test_star_one() {
        let solver = Day13 {};
        assert_eq!(solver.star_one(TEST_DATA), "405");
    }
}
