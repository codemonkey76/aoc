use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_06 {
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

    fn parse(&mut self)
    {
        let lines = aoclib::read_lines(aoclib::get_input_path(self.name()));
        let (times, distances) = (get_numbers(&lines[0]), get_numbers(&lines[1]));
        (0..times.len()).for_each(|i| {
            self.races.push(Race {
                time: times[i],
                distance: distances[i]
            })
        });
    }

    fn part1(&mut self) -> Vec<String> {
        let power = self.races.iter().fold(1, |acc, race| acc * race.max_winners());

        aoclib::output(power)
    }

    fn part2(&mut self) -> Vec<String> {
        let r = Race::combine(self.races.clone());

        aoclib::output(r.max_winners())
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