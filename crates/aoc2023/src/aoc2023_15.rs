use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_15 {
    input: PathBuf,
    steps: Vec<String>
}

impl Aoc2023_15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_15 {
    fn name(&self) -> (usize, usize) {
        (2023, 15)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        self.steps = aoclib::read_full(&self.input).split(',').map(|s| s.to_string()).collect();
    }

    fn part1(&mut self) -> i64 {
        self.steps.iter().fold(0, |acc, item| acc + get_hash(item))
    }

    fn part2(&mut self) -> i64 {
        0
    }
}

fn get_hash(item: &str) -> i64 {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.
    item.chars().fold(0, |acc, ch| {
        let mut result = acc + (ch as u8) as i64;
        result *= 17;
        result % 256
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_15::new();

        day.set_input("crates/aoc2023/test/2023-15.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(1320, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_15::new();

        day.set_input("crates/aoc2023/test/2023-15.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(0, result);
    }
}