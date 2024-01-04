use std::collections::HashSet;
use std::path::PathBuf;
use itertools::Itertools;
use aoclib::{get_repo_root, Runner, transpose};

#[derive(Default)]
pub struct Aoc2023_11 {
    input: PathBuf,
    inflation: Option<usize>,
    map: Vec<Vec<bool>>,
    rows_to_expand: HashSet<usize>,
    columns_to_expand: HashSet<usize>,
}

impl Aoc2023_11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_galaxy_positions(&self) -> HashSet<(usize, usize)> {
        let inflation = self.inflation.unwrap_or(1);
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

    fn get_distances(&self) -> i64 {
        self.get_galaxy_positions().iter().combinations(2).map(|pair| {
            let (a, b) = pair.iter().cloned().collect_tuple().unwrap();
            manhattan_distance(a, b)
        }).sum::<usize>() as i64
    }

    fn set_inflation(&mut self, inflation: usize) {
        self.inflation = Some(inflation);
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
        if self.inflation.is_none() {
            self.set_inflation(2);
        }

        self.get_distances()
    }

    fn part2(&mut self) -> i64 {
        if self.inflation.is_none() {
            self.set_inflation(1_000_000);
        }

        self.get_distances()
    }
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let (x1, y1) = *a;
    let (x2, y2) = *b;

    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };

    dx + dy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_11::new();

        day.set_input("crates/aoc2023/test/2023-11.txt");
        day.parse();
        day.set_inflation(2);
        let result = day.part1();

        assert_eq!(374, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_11::new();

        day.set_input("crates/aoc2023/test/2023-11.txt");
        day.parse();
        day.set_inflation(100);
        let result = day.part2();

        assert_eq!(8410, result);
    }
}