use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_09 {
    input: PathBuf,
}

impl Aoc2023_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_09 {
    fn name(&self) -> (usize, usize) {
        (0, 0)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> u64 {
        0
    }

    fn part2(&mut self) -> u64 {
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