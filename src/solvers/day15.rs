use crate::Solver;
use std::collections::HashMap;

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn hash_algorithm(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, ch| (acc + ch as u32) * 17 % 256)
}

pub struct Day15;

impl Solver for Day15 {
    fn star_one(&self, input: &str) -> String {
        input
            .trim_end()
            .split(',')
            .map(hash_algorithm)
            .sum::<u32>()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();

        for instruction in input.trim_end().split(',') {
            let (label, focal_length) = instruction
                .split_once(|ch| char::is_ascii_punctuation(&ch))
                .unwrap();

            let box_number = hash_algorithm(label);
            let lenses = boxes.entry(box_number).or_default();

            if focal_length.is_empty() {
                lenses.retain(|lens| lens.label != label);
                continue;
            }

            let new_lens = Lens {
                label: label.to_string(),
                focal_length: focal_length.parse().unwrap(),
            };

            if let Some(lens_index) = lenses.iter().position(|lens| lens.label == new_lens.label) {
                lenses[lens_index].focal_length = new_lens.focal_length;
            } else {
                lenses.push(new_lens);
            }
        }

        boxes
            .iter()
            .map(|(box_number, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(lens_index, lens)| {
                        (box_number + 1) * (lens_index as u32 + 1) * lens.focal_length
                    })
                    .sum::<u32>()
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_star_one() {
        let solver = Day15 {};
        assert_eq!(solver.star_one("HASH"), "52");
        assert_eq!(solver.star_one(TEST_DATA), "1320");
    }

    #[test]
    fn test_star_two() {
        let solver = Day15 {};
        assert_eq!(solver.star_two(TEST_DATA), "145");
    }
}
