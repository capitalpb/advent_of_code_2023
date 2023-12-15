use crate::Solver;

#[derive(Debug)]
struct PlatformMap {
    tiles: Vec<Vec<char>>,
}

impl PlatformMap {
    fn from(input: &str) -> PlatformMap {
        PlatformMap {
            tiles: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn load(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles.iter().filter(|tile| *tile == &'O').count() * (self.tiles.len() - row)
            })
            .sum()
    }

    fn tilt_north(&mut self) {
        for row in 1..self.tiles.len() {
            for col in 0..self.tiles[0].len() {
                if self.tiles[row][col] != 'O' {
                    continue;
                }

                let mut new_row = row;
                for check_row in (0..row).rev() {
                    if self.tiles[check_row][col] == '.' {
                        new_row = check_row;
                    } else {
                        break;
                    }
                }

                self.tiles[row][col] = '.';
                self.tiles[new_row][col] = 'O';
            }
        }
    }

    fn tilt_west(&mut self) {
        for col in 1..self.tiles[0].len() {
            for row in 0..self.tiles.len() {
                if self.tiles[row][col] != 'O' {
                    continue;
                }

                let mut new_col = col;
                for check_col in (0..col).rev() {
                    if self.tiles[row][check_col] == '.' {
                        new_col = check_col;
                    } else {
                        break;
                    }
                }

                self.tiles[row][col] = '.';
                self.tiles[row][new_col] = 'O';
            }
        }
    }

    fn tilt_south(&mut self) {
        for row in (0..(self.tiles.len() - 1)).rev() {
            for col in 0..self.tiles[0].len() {
                if self.tiles[row][col] != 'O' {
                    continue;
                }

                let mut new_row = row;
                for check_row in (row + 1)..self.tiles.len() {
                    if self.tiles[check_row][col] == '.' {
                        new_row = check_row;
                    } else {
                        break;
                    }
                }

                self.tiles[row][col] = '.';
                self.tiles[new_row][col] = 'O';
            }
        }
    }

    fn tilt_east(&mut self) {
        for col in (0..(self.tiles[0].len() - 1)).rev() {
            for row in 0..self.tiles.len() {
                if self.tiles[row][col] != 'O' {
                    continue;
                }

                let mut new_col = col;
                for check_col in (col + 1)..self.tiles[0].len() {
                    if self.tiles[row][check_col] == '.' {
                        new_col = check_col;
                    } else {
                        break;
                    }
                }

                self.tiles[row][col] = '.';
                self.tiles[row][new_col] = 'O';
            }
        }
    }
}

pub struct Day14;

impl Solver for Day14 {
    fn star_one(&self, input: &str) -> String {
        let mut platform_map = PlatformMap::from(input);
        platform_map.tilt_north();
        platform_map.load().to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let mut platform_map = PlatformMap::from(input);
        let mut map_history: Vec<Vec<Vec<char>>> = vec![];

        for index in 0..1_000_000_000 {
            platform_map.tilt_north();
            platform_map.tilt_west();
            platform_map.tilt_south();
            platform_map.tilt_east();

            if let Some(repeat_start) = map_history
                .iter()
                .position(|tiles| tiles == &platform_map.tiles)
            {
                let repeat_length = index - repeat_start;
                let delta = (1_000_000_000 - repeat_start) % repeat_length;
                let solution_index = repeat_start + delta - 1;

                return PlatformMap {
                    tiles: map_history[solution_index].clone(),
                }
                .load()
                .to_string();
            }

            map_history.push(platform_map.tiles.clone());
        }

        platform_map.load().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_star_one() {
        let solver = Day14 {};
        assert_eq!(solver.star_one(TEST_DATA), "136");
    }

    #[test]
    fn test_star_two() {
        let solver = Day14 {};
        assert_eq!(solver.star_two(TEST_DATA), "64");
    }
}
