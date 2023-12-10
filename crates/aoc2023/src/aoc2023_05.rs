use std::collections::{HashMap, HashSet};
use aoclib::Runner;
use itertools::Itertools;

#[derive(Default)]
pub struct Aoc2023_05 {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map
}

impl Aoc2023_05 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_location(seed: u64) -> u64 {

        0
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

        let mut maps: Vec<Map> = Vec::new();
        let mut current_map =  Map::default();

        for line in lines[2..].iter() {
            if line.contains(':') {
                maps.push(current_map);
                current_map = Map::default();
            } else {
                let tuple: (u64, u64, u64) = line
                    .split(' ')
                    .map(|num| num.trim().parse::<u64>().unwrap())
                    .collect_tuple().unwrap();

                current_map.lines.push(MapLine::from(tuple));
            }
        }

        maps.push(current_map);

        self.seed_to_soil = maps.remove(0);
        self.soil_to_fertilizer = maps.remove(0);
        self.fertilizer_to_water = maps.remove(0);
        self.water_to_light = maps.remove(0);
        self.light_to_temperature = maps.remove(0);
        self.temperature_to_humidity = maps.remove(0);
        self.humidity_to_location = maps.remove(0);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut seed_to_location : HashMap<u64, u64> = HashMap::new();

        for seed in &self.seeds {
            let soil = self.seed_to_soil.get_destination(*seed);
            let fertilizer = self.soil_to_fertilizer.get_destination(soil);
            let water = self.fertilizer_to_water.get_destination(fertilizer);
            let light = self.water_to_light.get_destination(water);
            let temperature = self.light_to_temperature.get_destination(light);
            let humidity = self.temperature_to_humidity.get_destination(temperature);
            let location = self.humidity_to_location.get_destination(humidity);


            seed_to_location.insert(*seed, location);
        }

        let min_location = seed_to_location.iter().min_by_key(|(_, &location)| location).map(|(_, &location)| location).unwrap();


        aoclib::output(min_location)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("Not Implemented")
    }
}

#[derive(Debug, Default)]
struct MapLine {
    source: u64,
    destination: u64,
    length: u64
}

impl From<(u64, u64, u64)> for MapLine {
    fn from(value: (u64, u64, u64)) -> Self {
        MapLine {
            source: value.1,
            destination: value.0,
            length: value.2
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    lines: Vec<MapLine>
}

impl Map {
    fn get_destination(&self, source: u64) -> u64 {
        let mut output: u64 = 0;

        for line in &self.lines {
            if line.source <= source && line.length >= (source.saturating_sub(line.source)) {
                output = line.destination + source.saturating_sub(line.source);
            }
        }
        if output == 0 {
            output = source
        }

        output
    }
}

