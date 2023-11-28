mod solvers;

use crate::solvers::*;

pub const LATEST_DAY: u8 = 1;

pub trait Solver {
    fn star_one(&self, input: &str) -> String;
    fn star_two(&self, input: &str) -> String;
}

pub fn init_solver(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(day01::Day01 {})),
        _ => None,
    }
}
