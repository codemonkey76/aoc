use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_06 {
    input: PathBuf,
    races: Vec<Race>
}

impl Aoc2023_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_06 {

    fn name(&self) -> (usize, usize) {
        (2023, 6)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self)
    {
        let lines = aoclib::read_lines(&self.input);
        let (times, distances) = (get_numbers(&lines[0]), get_numbers(&lines[1]));
        (0..times.len()).for_each(|i| {
            self.races.push(Race {
                time: times[i],
                distance: distances[i]
            })
        });
    }

    fn part1(&mut self) -> u64 {
        self.races.iter().fold(1, |acc, race| acc * race.max_winners())
    }

    fn part2(&mut self) -> u64 {
        Race::combine(self.races.clone()).max_winners()
    }
}

fn get_numbers(str: &str) -> Vec<u64> {
    str
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

#[derive(Debug, Default, Clone)]
struct Race {
    time: u64,
    distance: u64
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Race { time, distance }
    }

    fn combine(races: Vec<Race>) -> Race {
        races
            .iter()
            .fold(Race::default(), |acc, race| {
                let time_digits = Race::get_digits(race.time);
                let distance_digits = Race::get_digits(race.distance);

                Race::new(
                    acc.time * 10u64.pow(time_digits) + race.time,
                    acc.distance * 10u64.pow(distance_digits) + race.distance)
            })
    }

    fn get_digits(num: u64) -> u32 {
        (num as f64).log10() as u32 + 1
    }

    fn max_winners(&self) -> u64 {
        let mut winners = 0;
        (1..self.time).for_each(|i| {
            if i*(self.time-i)>self.distance {
                winners += 1;
            }
        });
        winners
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2023_06::Aoc2023_06;
    use super::*;

    #[test]
    fn part1() {
        let mut day6 = Aoc2023_06::new();

        day6.set_input("crates/aoc2023/test/2023-06.txt");
        day6.parse();
        let result = day6.part1();

        assert_eq!(288, result);


    }

    #[test]
    fn part2() {
        let mut day6 = Aoc2023_06::new();

        day6.set_input("crates/aoc2023/test/2023-06.txt");
        day6.parse();
        let result = day6.part2();
        assert_eq!(71503, result);
    }
}