use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc20xx_xx {
    input: PathBuf,
}

impl Aoc20xx_xx {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc20xx_xx {
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
    use crate::skeleton::Aoc20xx_xx;
    use super::*;

    #[test]
    fn part1() {
        let mut dayx = Aoc20xx_xx::new();

        dayx.set_input("crates/aoc20xx/test/20xx-xx.txt");
        dayx.parse();
        let result = dayx.part1();

        assert_eq!(0, result);
    }

    #[test]
    fn part2() {
        let mut dayx = Aoc20xx_xx::new();

        dayx.set_input("crates/aoc20xx/test/20xx-xx.txt");
        dayx.parse();
        let result = dayx.part2();

        assert_eq!(0, result);
    }
}