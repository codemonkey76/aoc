use std::path::PathBuf;
use aho_corasick::AhoCorasick;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_01 {
    input: PathBuf,
    lines: Vec<String>,
}

impl Aoc2023_01 {
    pub fn new() -> Self { Self::default() }
}

impl Runner for Aoc2023_01 {

    fn name(&self) -> (usize, usize) {
        (2023, 1)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        println!("Opening: {:?}", &self.input);
        self.lines = aoclib::read_lines(&self.input);
    }

    fn part1(&mut self) -> u64 {
        let mut total = 0;

        for line in &self.lines {
            let nums = line
                .chars()
                .filter(|ch| ch.is_ascii_digit())
                .map(|ch| (ch as u8 - b'0') as u64)
                .collect::<Vec<u64>>();
            let first = *nums.first().unwrap();
            let last = *nums.last().unwrap();

            total += first * 10 + last;
        }

        total
    }

    fn part2(&mut self) -> u64 {
        let nums = ["one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"];
        let mut total: u64 = 0;

        let ac = AhoCorasick::new(nums).unwrap();

        for line in &self.lines {
            let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
            let first = matches.get(0).unwrap().pattern().as_i32() / 2 + 1;
            let last = matches.iter().last().unwrap().pattern().as_i32() / 2 + 1;

            total += (10 * first + last) as u64;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day1 = Aoc2023_01::new();

        day1.set_input("crates/aoc2023/test/2023-01a.txt");
        day1.parse();
        let result = day1.part1();

        assert_eq!(142, result);


    }

    #[test]
    fn part2() {
        let mut day1 = Aoc2023_01::new();

        day1.set_input("crates/aoc2023/test/2023-01b.txt");
        day1.parse();
        let result = day1.part2();
        assert_eq!(281, result);
    }
}