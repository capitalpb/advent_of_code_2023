use crate::Solver;
use itertools::Itertools;
use num::abs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance_from(&self, other: &Point) -> i64 {
        abs(self.x as i64 - other.x as i64) + abs(self.y as i64 - other.y as i64)
    }
}

struct GalaxyMap {
    locations: Vec<Point>,
}

impl GalaxyMap {
    fn from(input: &str) -> GalaxyMap {
        let locations = input
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(x, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(|(y, digit)| {
                        if digit == '#' {
                            Some(Point { x, y })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        GalaxyMap { locations }
    }

    fn empty_rows(&self) -> Vec<usize> {
        let occupied_rows = self
            .locations
            .iter()
            .map(|point| point.y)
            .unique()
            .collect::<Vec<_>>();
        let max_y = *occupied_rows.iter().max().unwrap();

        (0..max_y)
            .filter(move |y| !occupied_rows.contains(y))
            .collect()
    }

    fn empty_cols(&self) -> Vec<usize> {
        let occupied_cols = self
            .locations
            .iter()
            .map(|point| point.x)
            .unique()
            .collect::<Vec<_>>();
        let max_x = *occupied_cols.iter().max().unwrap();

        (0..max_x)
            .filter(move |x| !occupied_cols.contains(x))
            .collect()
    }

    fn expand(&mut self, factor: usize) {
        let delta = factor - 1;

        for y in self.empty_rows().iter().rev() {
            for galaxy in &mut self.locations {
                if galaxy.y > *y {
                    galaxy.y += delta;
                }
            }
        }

        for x in self.empty_cols().iter().rev() {
            for galaxy in &mut self.locations {
                if galaxy.x > *x {
                    galaxy.x += delta;
                }
            }
        }
    }

    fn galactic_distance(&self) -> i64 {
        self.locations
            .iter()
            .combinations(2)
            .map(|pair| pair[0].distance_from(pair[1]))
            .sum::<i64>()
    }
}

pub struct Day11;

impl Solver for Day11 {
    fn star_one(&self, input: &str) -> String {
        let mut galaxy = GalaxyMap::from(input);
        galaxy.expand(2);
        galaxy.galactic_distance().to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let mut galaxy = GalaxyMap::from(input);
        galaxy.expand(1_000_000);
        galaxy.galactic_distance().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_star_one() {
        let solver = Day11 {};
        assert_eq!(solver.star_one(TEST_DATA), "374");
    }

    #[test]
    fn test_expansion_factor_10() {
        let mut galaxy = GalaxyMap::from(TEST_DATA);
        galaxy.expand(10);
        assert_eq!(galaxy.galactic_distance(), 1030);
    }

    #[test]
    fn test_expansion_factor_100() {
        let mut galaxy = GalaxyMap::from(TEST_DATA);
        galaxy.expand(100);
        assert_eq!(galaxy.galactic_distance(), 8410);
    }
}
