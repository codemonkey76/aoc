use std::ops::Range;
use aoclib::Runner;
use itertools::Itertools;

#[derive(Default)]
pub struct Aoc2023_05 {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>,
}

impl Aoc2023_05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_05 {
    fn name(&self) -> (usize, usize) {
        (2023, 5)
    }

    fn parse(&mut self)
    {
        let lines = aoclib::read_lines(aoclib::get_input_path(self.name()));

        self.seeds = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .map(|num| num.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut current_map =  Mapping::default();

        for line in lines[2..].iter() {
            if line.contains(':') {
                self.mappings.push(current_map);
                current_map = Mapping::default();
            } else {
                let tuple: (i64, i64, i64) = line
                    .split(' ')
                    .map(|num| num.trim().parse::<i64>().unwrap())
                    .collect_tuple().unwrap();

                current_map.map.push(SingleMap::from(tuple));
            }
        }

        self.mappings.push(current_map);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut min_location = i64::MAX;

        for seed in &self.seeds {
            let mut current = *seed;
            for mapping in &self.mappings {
                current = mapping.apply_map(current);
            }

            min_location = min_location.min(current);
        }

        aoclib::output(min_location)
    }

    fn part2(&mut self) -> Vec<String> {
        let seed_ranges = self.seeds
            .chunks(2).map(|vec| Range {
                start: vec[0],
                end: vec[0] + vec[1],
            })
            .collect::<Vec<_>>();

        let mut location: i64 = 1_i64;

        loop {

            let mut current = location;
            for mapping in self.mappings.iter().rev() {
                current = mapping.reverse_lookup(current);
            }

            for seed_range in &seed_ranges {
                if seed_range.contains(&current) {
                    return aoclib::output(location);
                }
            }
            location += 1;

            if location == i64::MAX {
                panic!("Couldn't find a location");
            }
        }
    }
}

#[derive(Debug, Default)]
struct Mapping {
    map: Vec<SingleMap>
}

impl Mapping {
    fn apply_map(&self, value: i64) -> i64 {
        for map in &self.map {
            if map.range.contains(&value) {
                return value-map.delta;
            }
        }
        value
    }
    fn reverse_lookup(&self, value: i64) -> i64 {
        for map in &self.map {
            let rev = value + map.delta;
            if map.range.contains(&rev) {
                return rev;
            }
        }
        value
    }
}

#[derive(Debug, Default, Clone)]
struct SingleMap {
    range: Range<i64>,
    delta: i64
}


impl From<(i64, i64, i64)> for SingleMap {
    fn from(value: (i64, i64, i64)) -> Self {
        SingleMap {
            range: Range {
                start: value.1,
                end: value.1+value.2
            },
            delta: value.1-value.0
        }
    }
}
