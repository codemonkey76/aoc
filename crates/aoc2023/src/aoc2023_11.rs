use std::path::PathBuf;
use itertools::Itertools;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_11 {
    input: PathBuf,
    map: Vec<Vec<bool>>,
    expanded_map: Vec<Vec<bool>>,
    galaxies: Vec<(usize, usize)>
}

impl Aoc2023_11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn draw_map(&self, map: &[Vec<bool>]) {
        map.iter().for_each(|row| {
            row.iter().for_each(|&col| if col { print!("#"); } else { print!("."); });
            println!();
        });
    }
}

impl Runner for Aoc2023_11 {
    fn name(&self) -> (usize, usize) {
        (2023, 11)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines(&self.input);
        self.map = lines.iter().map(|line| {
            line.chars().map(|c| c == '#').collect()
        }).collect();

        // Expand Rows
        self.map.iter().for_each(| row| {
            if row.iter().all(|pos| !pos) {
                self.expanded_map.push(row.clone());
            }
            self.expanded_map.push(row.clone());
        });

        // Expand Cols
        let mut new_map = vec![];
        let transposed_map = transpose(&self.expanded_map);
        transposed_map.iter().for_each(|row| {
            if row.iter().all(|pos| !pos) {
                new_map.push(row.clone());
            }
            new_map.push(row.clone());
        });

        self.expanded_map = transpose(&new_map);

        // Get Galaxy Positions.

        self.expanded_map.iter().enumerate().for_each(|(row_index, row)| {
           row.iter().enumerate().for_each(|(col_index, &val)| {
               if val {
                   self.galaxies.push((row_index, col_index));
               }
           });
        });
    }

    fn part1(&mut self) -> i64 {
        self.galaxies.iter().combinations(2).map(|pair| {
           let (a, b) = pair.iter().cloned().collect_tuple().unwrap();
            get_distance(a, b)
        }).sum()
    }

    fn part2(&mut self) -> i64 {
        0
    }
}

fn get_distance(a: &(usize, usize), b: &(usize, usize)) -> i64 {
    let mut steps = 0;
    let mut a = *a;

    while a.0 < b.0 {
        a.0 += 1;
        steps += 1;
    }

    while a.0 > b.0 {
        a.0 -= 1;
        steps += 1;
    }

    while a.1 < b.1 {
        a.1 += 1;
        steps += 1;
    }

    while a.1 > b.1 {
        a.1 -= 1;
        steps += 1;
    }

    steps
}

fn transpose(map: &[Vec<bool>]) -> Vec<Vec<bool>> {
    if map.is_empty() || map.iter().any(|row| row.len() != map[0].len()) {
        panic!("Invalid input: Empty vector or inconsistent row lengths");
    }

    let row_count = map.len();
    let col_count = map[0].len();

    (0..col_count).map(|col_index| (0..row_count).map(|row_index| map[row_index][col_index]).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_11::new();

        day.set_input("crates/aoc2023/test/2023-11.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(374, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_11::new();

        day.set_input("crates/aoc2023/test/2023-11.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(0, result);
    }
}