use std::collections::HashMap;
use aoclib::Runner;
use itertools::Itertools;

#[derive(Default)]
pub struct Aoc2023_05 {
    seeds: Vec<u64>,
    maps: HashMap<String, Mapping>
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
        let mut groups = aoclib::read_groups(aoclib::get_input_path(self.name()));
        self.seeds = parse_seeds(groups.remove(0));
        self.maps = parse_maps(groups);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut result = u64::MAX;

        for seed in &self.seeds {
            let mut current = *seed;
            for map_name in ["soil", "fertilizer", "water", "light", "temperature", "humidity", "location"] {
                let mapping = self.maps.get(map_name).unwrap();
                current = mapping.apply(current);
            }
            result = result.min(current);
        }

        aoclib::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut ranges: Vec<(u64, u64)> = vec![];

        for seed_pair in self.seeds.chunks(2) {
            ranges.push((seed_pair[0], seed_pair[0] + seed_pair[1] - 1));
        }

        ranges = apply_range_mappings(&mut ranges, &self.maps);
        ranges.sort();

        let output = ranges[0].0;
        aoclib::output(output)
    }
}

fn apply_range_mappings(ranges: &mut Vec<(u64, u64)>, maps: &HashMap<String, Mapping>)  -> Vec<(u64, u64)> {
    let mut ranges_clone = ranges.clone();

    for map_name in ["soil", "fertilizer", "water", "light", "temperature", "humidity", "location"] {
        let mapping = maps.get(map_name).unwrap();
        ranges_clone = apply_range_mapping(&mut ranges_clone, mapping);
    }
    ranges_clone
}

fn apply_range_mapping(ranges: &mut Vec<(u64, u64)>, map: &Mapping) -> Vec<(u64,u64)> {
    let mut new_ranges = vec![];
    let mut i = 0;

    while i < ranges.len() {
        let mut matched = false;
        for entity in &map.mappings {
            let entity_range = (entity.src, entity.src+entity.range);
            let os = ranges[i].0.max(entity_range.0);
            let oe = ranges[i].1.min(entity_range.1);
            if os < oe {
                new_ranges.push((os - entity.src+entity.dest, oe-entity.src+entity.dest));
                matched = true;
                // Check these again for other matches
                if os > ranges[i].0 { ranges.push((ranges[i].0, os)); }
                if ranges[i].1 > oe { ranges.push((oe, ranges[i].1)); }
                // If a match found we can break
                break;
            }
        }
        if !matched { new_ranges.push((ranges[i].0, ranges[i].1)); }
        i+=1;
    }
    new_ranges
}

fn parse_seeds(seeds: String) -> Vec<u64> {
    seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

fn parse_maps(mut groups: Vec<String>) -> HashMap<String, Mapping> {
    let mut maps:HashMap<String, Mapping> = HashMap::new();

    while !groups.is_empty() {
        let item = groups.remove(0);
        let mut contents = item.split('\n').filter(|str| !str.is_empty()).map(|str| str.to_string()).collect::<Vec<String>>();

        let map_row = contents.remove(0);
        let (_, _, to) = map_row.split_once(' ').unwrap().0.split('-').collect_tuple().unwrap();

        let mappings : Vec<MapRange> = contents
            .iter()
            .map(|line| {
                MapRange::from(line)
            })
            .collect::<Vec<MapRange>>();

        maps.insert(to.to_string(), Mapping { mappings });
    }
    maps
}

#[derive(Debug)]
struct Mapping {
    mappings: Vec<MapRange>
}
impl Mapping {
    fn apply(&self, current: u64) -> u64 {
        let mut value: u64 = current;

        for entity in &self.mappings {
            if value >= entity.src && value < entity.src + entity.range {
                value = entity.dest + (value - entity.src);
                break;
            }
        }
        value
    }
}

#[derive(Debug)]
struct MapRange {
    src: u64,
    dest: u64,
    range: u64
}
impl From<(u64,u64,u64)> for MapRange {
    fn from(value: (u64, u64, u64)) -> Self {
        MapRange {
            src: value.1,
            dest: value.0,
            range: value.2
        }
    }
}
impl From<&String> for MapRange {
    fn from(value: &String) -> Self {
        MapRange::from(
            value
                .split(' ')
                .map(|num| num.parse::<u64>().unwrap())
                .collect_tuple::<(u64,u64,u64)>()
                .unwrap()
        )
    }
}