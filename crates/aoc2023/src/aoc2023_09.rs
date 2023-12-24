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
        self.numbers.iter().fold(Vec::new(), |mut acc, numbers| {
            get_next_number(&mut acc, numbers);
            acc
        }).iter().sum()
    }


    fn part2(&mut self) -> i64 {
        self.numbers.iter().fold(Vec::new(), |mut acc, numbers| {
            get_previous_number(&mut acc, numbers);
            acc
        }).iter().sum()
    }
}

fn get_previous_number(acc: &mut Vec<i64>, numbers: &[i64]) {
    let mut tree = build_tree(numbers);

    let mut first_item = 0i64;

    for item in tree.iter_mut().rev() {
        item.insert(0, item.first().unwrap() - first_item);
        first_item = *item.first().unwrap();
    }

    acc.push(first_item)
}

fn build_tree(numbers: &[i64]) -> Vec<Vec<i64>>{
    let mut current = numbers.to_vec();
    let mut tree : Vec<Vec<i64>> = Vec::new();

    tree.push(current.clone());

    while !current.iter().all(|&x| x == 0) {
        current = get_differences(&current);
        tree.push(current.clone());
    }
    tree
}

fn get_next_number(acc: &mut Vec<i64>, numbers: &[i64]) {
    let mut tree = build_tree(numbers);

    let mut last_item = 0i64;
    for item in tree.iter_mut().rev() {
        item.push(last_item + item.last().unwrap());
        last_item = *item.last().unwrap();
    }

    acc.push(last_item);
}

fn get_differences(numbers: &[i64]) -> Vec<i64> {
    numbers.windows(2).map(|w| w[1] - w[0]).collect()
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

        assert_eq!(114, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_09::new();

        day.set_input("crates/aoc2023/test/2023-09.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(2, result);
    }
}

