use crate::Solver;

#[derive(Clone, Copy, Debug)]
struct Location {
    row: usize,
    start_col: usize,
    end_col: usize,
}

#[derive(Debug)]
struct EngineSchematic {
    schematic: Vec<Vec<char>>,
    numbers: Vec<Location>,
    symbols: Vec<Location>,
}

impl EngineSchematic {
    fn from(input: &str) -> EngineSchematic {
        let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut numbers = vec![];
        let mut symbols = vec![];
        let mut location: Option<Location> = None;

        for (row_index, row) in schematic.iter().enumerate() {
            for (col, ch) in row.iter().enumerate() {
                match ch {
                    ch if ch.is_ascii_punctuation() => {
                        if let Some(location) = location {
                            numbers.push(location);
                        }
                        location = None;

                        if ch != &'.' {
                            symbols.push(Location {
                                row: row_index,
                                start_col: col,
                                end_col: col,
                            });
                        }
                    }
                    ch if ch.is_ascii_digit() => {
                        if let Some(mut_location) = location.as_mut() {
                            mut_location.end_col = col;
                        } else {
                            location = Some(Location {
                                row: row_index,
                                start_col: col,
                                end_col: col,
                            });
                        }
                    }
                    _ => {
                        unreachable!("malformed input");
                    }
                }
            }

            if let Some(location) = location {
                numbers.push(location);
            }
            location = None;
        }

        EngineSchematic {
            schematic,
            numbers,
            symbols,
        }
    }

    fn get_number_value(&self, location: &Location) -> u32 {
        self.schematic[location.row][location.start_col..=location.end_col]
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    }

    fn is_gear(&self, location: &Location) -> bool {
        self.schematic[location.row][location.start_col] == '*'
    }

    fn are_adjacent(&self, location: &Location, other_location: &Location) -> bool {
        location.start_col >= other_location.start_col.saturating_sub(1)
            && location.end_col <= other_location.end_col + 1
            && (location.row == other_location.row
                || location.row == other_location.row.saturating_sub(1)
                || location.row == other_location.row + 1)
    }
}

pub struct Day03;

impl Solver for Day03 {
    fn star_one(&self, input: &str) -> String {
        let schematic = EngineSchematic::from(input);

        schematic
            .numbers
            .iter()
            .filter(|number| {
                schematic
                    .symbols
                    .iter()
                    .any(|symbol| schematic.are_adjacent(symbol, number))
            })
            .map(|number| schematic.get_number_value(number))
            .sum::<u32>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let schematic = EngineSchematic::from(input);

        schematic
            .symbols
            .iter()
            .filter(|symbol| schematic.is_gear(symbol))
            .map(|symbol| {
                let adjacent_numbers = schematic
                    .numbers
                    .iter()
                    .filter(|number| schematic.are_adjacent(symbol, number))
                    .collect::<Vec<_>>();
                if adjacent_numbers.len() == 2 {
                    schematic.get_number_value(adjacent_numbers[0])
                        * schematic.get_number_value(adjacent_numbers[1])
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_star_one() {
        let solver = Day03;
        assert_eq!(solver.star_one(TEST_DATA), "4361");
    }

    #[test]
    fn test_star_two() {
        let solver = Day03;
        assert_eq!(solver.star_two(TEST_DATA), "467835");
    }
}
