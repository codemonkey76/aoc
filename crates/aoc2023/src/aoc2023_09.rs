use std::path::PathBuf;
use aoclib::{get_repo_root, read_lines, Runner};

#[derive(Default)]
pub struct Aoc2023_09 {
    input: PathBuf,
    numbers: Vec<Vec<i64>>
}

impl Aoc2023_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_09 {
    fn name(&self) -> (usize, usize) {
        (2023, 9)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        println!("{:?}", self.input);

        let lines = read_lines(&self.input);
        self.numbers = lines
            .iter()
            .map(|line|
                line
                    .split_whitespace()
                    .map(|num|
                        num
                            .parse::<i64>()
                            .unwrap()
                    ).collect::<Vec<i64>>()
            ).collect();
    }

    fn part1(&mut self) -> i64 {
        0
    }

    fn part2(&mut self) -> i64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_09::new();

        day.set_input("crates/aoc2023/test/2023-09.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(0, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_09::new();

        day.set_input("crates/aoc2023/test/2023-09.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(0, result);
    }
}