use std::collections::HashSet;
use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_03 {
    input: PathBuf,
    numbers: Vec<PartNumber>,
    symbols: HashSet<(i64, i64)>,
    gears: HashSet<(i64, i64)>
}

impl Aoc2023_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_03 {
    fn name(&self) -> (usize, usize) {
        (2023, 3)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines(&self.input);

        let mut cur_number: Option<PartNumber> = None;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    if let Some(ref mut num) = cur_number {
                        num.add_digit(row as i64, col as i64, ch);
                    } else {
                        cur_number = Some(PartNumber::new(row as i64, col as i64, ch));
                    }
                } else {
                    if let Some(num) = cur_number.take() {
                        self.numbers.push(num);
                    }
                    if ch != '.' {
                        self.symbols.insert((row as i64, col as i64));
                        if ch == '*' {
                            self.gears.insert((row as i64, col as i64));
                        }
                    }
                }
            }
        }
    }

    fn part1(&mut self) -> i64 {
        let total = self.numbers
            .iter()
            .filter(|num| num.next_to_symbol(&self.symbols))
            .map(|num| num.value)
            .sum();

        total
    }

    fn part2(&mut self) -> i64 {
        let mut total = 0;

        'next_gear: for gear in &self.gears {
            let mut matches: Vec<i64> = Vec::new();
            for num in &self.numbers {
                if num.points.contains(gear) {
                    if matches.len() == 2 {
                        continue 'next_gear;
                    }
                    matches.push(num.value);
                }
            }
            if matches.len() == 2 {
                total += matches[0] * matches[1];
            }
        }
        total
    }
}

#[derive(Debug)]
struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row-1, col-1), (row, col-1), (row+1, col-1),   // Left hand side
            (row-1, col), (row+1, col),                     // above and below
            (row-1, col+1), (row, col+1), (row+1, col+1)    // Right hand side
        ]);

        Self {
            value: (ch as u8 - b'0') as i64,
            points
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
        self.points.extend([
            (row-1, col+1), (row, col+1), (row+1, col+1)
        ]);
    }

    fn next_to_symbol(&self, symbols: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(symbols).next().is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2023_03::Aoc2023_03;
    use super::*;

    #[test]
    fn part1() {
        let mut day3 = Aoc2023_03::new();

        day3.set_input("crates/aoc2023/test/2023-03.txt");
        day3.parse();
        let result = day3.part1();

        assert_eq!(4361, result);


    }

    #[test]
    fn part2() {
        let mut day3 = Aoc2023_03::new();

        day3.set_input("crates/aoc2023/test/2023-03.txt");
        day3.parse();
        let result = day3.part2();
        assert_eq!(467835, result);
    }
}