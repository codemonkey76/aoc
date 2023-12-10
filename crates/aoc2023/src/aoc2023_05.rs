use std::ops::Range;
use aoclib::Runner;
use itertools::Itertools;

#[derive(Default)]
pub struct Aoc2023_05 {
    seeds: Vec<u64>,
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
            .map(|num| num.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mut current_map =  Mapping::default();

        for line in lines[2..].iter() {
            if line.contains(':') {
                self.mappings.push(current_map);
                current_map = Mapping::default();
            } else {
                let tuple: (u64, u64, u64) = line
                    .split(' ')
                    .map(|num| num.trim().parse::<u64>().unwrap())
                    .collect_tuple().unwrap();

                current_map.map.push(SingleMap::from(tuple));
            }
        }

        self.mappings.push(current_map);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut min_location = u64::MAX;

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
        let mut min_location: u64 = u64::MAX;

        for seed_range in self.seeds.chunks(2) {
            for seed in seed_range[0]..seed_range[0]+seed_range[1] {
                let mut current = seed;
                for mapping in &self.mappings {
                    current = mapping.apply_map(current);
                }

                min_location = min_location.min(current);
            }
        }
        // for (index, seed) in self.seeds.iter().enumerate() {
        //     let mut location: u64;
        //
        //     if index % 2 == 0 {
        //         location = self.get_location(*seed);
        //         lowest_location = location.min(lowest_location);
        //         last_seed = *seed;
        //     }
        //     else {
        //         for i in last_seed..*seed+last_seed-1 {
        //             location = self.get_location(i);
        //             lowest_location = location.min(lowest_location);
        //         }
        //
        //     }
        // }

        aoclib::output(min_location)
    }
}

#[derive(Debug, Default)]
struct Mapping {
    map: Vec<SingleMap>
}

impl Mapping {
    fn apply_map(&self, value: u64) -> u64 {
        for map in &self.map {
            if map.range.contains(&value) {
                return value+map.delta;
            }
        }
        value
    }
}

#[derive(Debug, Default, Clone)]
struct SingleMap {
    range: Range<u64>,
    delta: u64
}


impl From<(u64, u64, u64)> for SingleMap {
    fn from(value: (u64, u64, u64)) -> Self {
        SingleMap {
            range: Range {
                start: value.1,
                end: value.1+value.2
            },
            delta: 1
        }
    }
}
