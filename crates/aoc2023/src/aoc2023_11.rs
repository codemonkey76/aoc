use std::collections::HashSet;
use std::path::PathBuf;
use itertools::Itertools;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_11 {
    input: PathBuf,
    map: Vec<Vec<bool>>,
    rows_to_expand: HashSet<usize>,
    columns_to_expand: HashSet<usize>,
}

impl Aoc2023_11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_galaxy_positions(&self, inflation: usize) -> HashSet<(usize, usize)> {
        let mut galaxies: HashSet<(usize, usize)> = HashSet::new();

        self.map.iter().enumerate().for_each(|(row_index, row)| {
            let rows = self.rows_to_expand.iter().filter(|&x| *x < row_index).count();
            row.iter().enumerate().for_each(|(col_index, &val)| {
                if val {
                    let cols = self.columns_to_expand.iter().filter(|&x| *x < col_index).count();
                    galaxies.insert(((rows*inflation)+row_index-rows, (cols*inflation)+col_index-cols));
                }
            });
        });
        galaxies
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

        // get blank Rows
        self.map.iter().enumerate().for_each(| (row_index, row)| {
            if row.iter().all(|pos| !pos) {
                self.rows_to_expand.insert(row_index);
            }
        });

        // get blank Cols
        let transposed_map = transpose(&self.map);
        transposed_map.iter().enumerate().for_each(|(col_index, col)| {
            if col.iter().all(|pos| !pos) {
                self.columns_to_expand.insert(col_index);
            }
        });

    }

    fn part1(&mut self) -> i64 {
        self.get_galaxy_positions(2).iter().combinations(2).map(|pair| {
           let (a, b) = pair.iter().cloned().collect_tuple().unwrap();
            manhattan_distance(a, b)
        }).sum::<usize>() as i64
    }

    fn part2(&mut self) -> i64 {
        self.get_galaxy_positions(1000000).iter().combinations(2).map(|pair| {
            let (a, b) = pair.iter().cloned().collect_tuple().unwrap();
            manhattan_distance(a, b)
        }).sum::<usize>() as i64
    }
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let (x1, y1) = *a;
    let (x2, y2) = *b;

    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };

    dx + dy
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