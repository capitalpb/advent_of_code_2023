use crate::Solver;

#[derive(Debug)]
struct PipeMap {
    start: usize,
    tiles: Vec<char>,
    width: usize,
}

impl PipeMap {
    fn from(input: &str) -> PipeMap {
        let tiles = input
            .lines()
            .rev()
            .flat_map(|row| row.chars())
            .collect::<Vec<_>>();

        let width = input.find('\n').unwrap();
        let start = tiles.iter().position(|tile| tile == &'S').unwrap();

        PipeMap {
            start,
            tiles,
            width,
        }
    }

    fn valid_steps(&self, index: usize) -> Vec<usize> {
        let mut tiles = vec![];
        let current_tile = *self.tiles.get(index).unwrap();

        if "S|LJ".contains(current_tile) {
            let north = index + self.width;
            if let Some(tile) = self.tiles.get(north) {
                if "|7F".contains(*tile) {
                    tiles.push(north);
                }
            }
        }

        if "S|7F".contains(current_tile) {
            if let Some(south) = index.checked_sub(self.width) {
                if let Some(tile) = self.tiles.get(south) {
                    if "|LJ".contains(*tile) {
                        tiles.push(south);
                    }
                }
            }
        }

        if "S-J7".contains(current_tile) {
            if let Some(west) = index.checked_sub(1) {
                if (west % self.width) != (self.width - 1) {
                    if let Some(tile) = self.tiles.get(west) {
                        if "-LF".contains(*tile) {
                            tiles.push(west);
                        }
                    }
                }
            }
        }

        if "S-LF".contains(current_tile) {
            let east = index + 1;
            if east % self.width != 0 {
                if let Some(tile) = self.tiles.get(east) {
                    if "-J7".contains(*tile) {
                        tiles.push(east);
                    }
                }
            }
        }

        tiles
    }
}

pub struct Day10;

impl Solver for Day10 {
    fn star_one(&self, input: &str) -> String {
        let pipe_map = PipeMap::from(input);

        let mut current_pos = pipe_map.start;
        let mut last_pos = pipe_map.start;
        let mut steps: usize = 0;

        'outer: loop {
            for pos in pipe_map.valid_steps(current_pos) {
                if pos != last_pos {
                    last_pos = current_pos;
                    current_pos = pos;
                    steps += 1;

                    continue 'outer;
                }
            }
            break;
        }

        steps.div_ceil(2).to_string()
    }

    fn star_two(&self, _input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn test_star_one() {
        let solver = Day10 {};
        assert_eq!(solver.star_one(TEST_DATA), "8");
    }
}
