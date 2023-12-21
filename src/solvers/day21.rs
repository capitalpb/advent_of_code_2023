use crate::Solver;
use array2d::Array2D;
use std::collections::HashSet;

pub struct Day21;

impl Solver for Day21 {
    fn star_one(&self, input: &str) -> String {
        let rows = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let garden_map = Array2D::from_rows(&rows).unwrap();

        let start = garden_map
            .elements_row_major_iter()
            .position(|tile| *tile == 'S')
            .unwrap();
        let start = garden_map.indices_row_major().nth(start).unwrap();

        let mut current_plots = HashSet::new();
        current_plots.insert(start);

        for _ in 0..64 {
            let mut new_plots = HashSet::new();

            for plot in current_plots.drain() {
                if plot.0 != 0 && garden_map[(plot.0 - 1, plot.1)] != '#' {
                    new_plots.insert((plot.0 - 1, plot.1));
                }

                if plot.0 != (garden_map.row_len() - 1) && garden_map[(plot.0 + 1, plot.1)] != '#' {
                    new_plots.insert((plot.0 + 1, plot.1));
                }

                if plot.1 != 0 && garden_map[(plot.0, plot.1 - 1)] != '#' {
                    new_plots.insert((plot.0, plot.1 - 1));
                }

                if plot.1 != (garden_map.column_len() - 1)
                    && garden_map[(plot.0, plot.1 + 1)] != '#'
                {
                    new_plots.insert((plot.0, plot.1 + 1));
                }
            }

            current_plots = new_plots;
        }

        current_plots.len().to_string()
    }

    fn star_two(&self, _input: &str) -> String {
        todo!()
    }
}
