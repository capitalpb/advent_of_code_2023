use crate::Solver;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
struct AlmanacMapEntry {
    destination_range: RangeInclusive<u64>,
    source_range: RangeInclusive<u64>,
}

#[derive(Clone, Debug)]
struct AlmanacMap {
    entries: Vec<AlmanacMapEntry>,
}

impl AlmanacMap {
    fn from(input: &str) -> AlmanacMap {
        let entries = input
            .lines()
            .skip(1)
            .map(|line| {
                let numbers = line
                    .split(' ')
                    .filter_map(|number| number.parse::<u64>().ok())
                    .collect::<Vec<_>>();
                AlmanacMapEntry {
                    destination_range: numbers[0]..=(numbers[0] + numbers[2]),
                    source_range: numbers[1]..=(numbers[1] + numbers[2]),
                }
            })
            .collect();
        AlmanacMap { entries }
    }

    fn convert(&self, source: &u64) -> u64 {
        let entry = self
            .entries
            .iter()
            .find(|entry| entry.source_range.contains(source));

        if let Some(entry) = entry {
            entry.destination_range.start() + (source - entry.source_range.start())
        } else {
            *source
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

impl Almanac {
    fn star_one_from(input: &str) -> Almanac {
        let mut input_sections = input
            .split("\n\n")
            .map(|section| section.split_once(':').unwrap().1);

        let seeds = input_sections
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|seed| seed.parse::<u64>().ok())
            .collect();

        let almanac_maps = input_sections.map(AlmanacMap::from).collect::<Vec<_>>();

        Almanac {
            seeds,
            seed_to_soil: almanac_maps[0].clone(),
            soil_to_fertilizer: almanac_maps[1].clone(),
            fertilizer_to_water: almanac_maps[2].clone(),
            water_to_light: almanac_maps[3].clone(),
            light_to_temperature: almanac_maps[4].clone(),
            temperature_to_humidity: almanac_maps[5].clone(),
            humidity_to_location: almanac_maps[6].clone(),
        }
    }

    fn star_two_from(input: &str) -> Almanac {
        let mut input_sections = input
            .split("\n\n")
            .map(|section| section.split_once(':').unwrap().1);

        let seeds = input_sections
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|seed| seed.parse::<u64>().ok())
            .collect::<Vec<_>>()
            .chunks(2)
            .flat_map(|chunk| (chunk[0]..(chunk[0] + chunk[1])).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let almanac_maps = input_sections.map(AlmanacMap::from).collect::<Vec<_>>();

        Almanac {
            seeds,
            seed_to_soil: almanac_maps[0].clone(),
            soil_to_fertilizer: almanac_maps[1].clone(),
            fertilizer_to_water: almanac_maps[2].clone(),
            water_to_light: almanac_maps[3].clone(),
            light_to_temperature: almanac_maps[4].clone(),
            temperature_to_humidity: almanac_maps[5].clone(),
            humidity_to_location: almanac_maps[6].clone(),
        }
    }
}

pub struct Day05;

impl Solver for Day05 {
    fn star_one(&self, input: &str) -> String {
        let almanac = Almanac::star_one_from(input);

        almanac
            .seeds
            .iter()
            .map(|seed| almanac.seed_to_soil.convert(seed))
            .map(|soil| almanac.soil_to_fertilizer.convert(&soil))
            .map(|fertilizer| almanac.fertilizer_to_water.convert(&fertilizer))
            .map(|water| almanac.water_to_light.convert(&water))
            .map(|light| almanac.light_to_temperature.convert(&light))
            .map(|temperature| almanac.temperature_to_humidity.convert(&temperature))
            .map(|humidity| almanac.humidity_to_location.convert(&humidity))
            .min()
            .unwrap()
            .to_string()
    }

    fn star_two(&self, input: &str) -> String {
        let almanac = Almanac::star_two_from(input);

        almanac
            .seeds
            .iter()
            .map(|seed| almanac.seed_to_soil.convert(seed))
            .map(|soil| almanac.soil_to_fertilizer.convert(&soil))
            .map(|fertilizer| almanac.fertilizer_to_water.convert(&fertilizer))
            .map(|water| almanac.water_to_light.convert(&water))
            .map(|light| almanac.light_to_temperature.convert(&light))
            .map(|temperature| almanac.temperature_to_humidity.convert(&temperature))
            .map(|humidity| almanac.humidity_to_location.convert(&humidity))
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_star_one() {
        let solver = Day05 {};
        assert_eq!(solver.star_one(TEST_DATA), "35");
    }

    #[test]
    fn test_star_two() {
        let solver = Day05 {};
        assert_eq!(solver.star_two(TEST_DATA), "46");
    }
}
