mod solvers;

use crate::solvers::*;

pub const LATEST_DAY: u8 = 4;

pub trait Solver {
    fn star_one(&self, input: &str) -> String;
    fn star_two(&self, input: &str) -> String;
}

pub fn get_solver(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(day01::Day01 {})),
        2 => Some(Box::new(day02::Day02 {})),
        3 => Some(Box::new(day03::Day03 {})),
        4 => Some(Box::new(day04::Day04 {})),
        _ => None,
    }
}
