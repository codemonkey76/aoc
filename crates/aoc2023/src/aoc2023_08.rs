use std::collections::HashMap;
use std::path::PathBuf;
use aoclib::{get_repo_root, lcm_of, Runner};

#[derive(Default)]
pub struct Aoc2023_08 {
    input: PathBuf,
    instructions: Vec<char>,
    nodes: HashMap<String, (String, String)>
}

impl Aoc2023_08 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, current: &String, count: u64) -> &String {
        let instruction = self.instructions[count as usize % self.instructions.len()];
        match instruction {
            'L' => &self.nodes.get(current).unwrap().0,
            'R' => &self.nodes.get(current).unwrap().1,
            _ => panic!("Invalid instruction")
        }
    }
}

impl Runner for Aoc2023_08 {
    fn name(&self) -> (usize, usize) {
        (2023, 8)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        let mut lines = aoclib::read_lines(&self.input);
        self.instructions = lines.remove(0).chars().collect();

        lines.iter().for_each(|line| {
            let (key, rest) = line.split_once(" = ").unwrap();
            let (left, right) = rest.trim_matches(|c| c == '(' || c == ')').split_once(", ").unwrap();
            self.nodes.insert(key.to_string(), (left.to_string(), right.to_string()));
        });
    }

    fn part1(&mut self) -> u64 {
        let mut steps: u64 = 0;
        let mut current = &"AAA".to_string();
        while current != "ZZZ" {
            current = self.get(current, steps);

            steps += 1;
        }
        steps
    }

    fn part2(&mut self) -> u64 {
        let mut counts: Vec<u64> = vec![];

        for entry in self.nodes.keys().filter(|item| item.ends_with('A')).collect::<Vec<&String>>() {
            let mut current = entry;
            let mut count: u64 = 0;

            while !current.ends_with('Z') {
                current = self.get(current, count);
                count += 1;
            }
            counts.push(count);
        }
        lcm_of(counts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_08::new();

        day.set_input("crates/aoc2023/test/2023-08a.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(6, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_08::new();

        day.set_input("crates/aoc2023/test/2023-08b.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(6, result);
    }
}