use std::path::PathBuf;
use itertools::Itertools;
use aoclib::{get_repo_root, read_lines, Runner};

#[derive(Default)]
pub struct Aoc2023_10 {
    input: PathBuf,
    maze: Vec<Vec<PipeShape>>,
    pipe: Vec<(usize, usize)>
}

impl Aoc2023_10 {
    pub fn new() -> Self {
        Self::default()
    }
    fn find_starting_pos(&self) -> Option<(usize, usize)> {
        for (row_index, row) in self.maze.iter().enumerate() {
            if let Some(col_index) = row.iter().position(|shape| *shape == PipeShape::StartingPosition) {
                return Some((row_index, col_index));
            }
        }
        None
    }

    fn get_shape(&self, pos: (usize, usize)) -> &PipeShape {
        self.maze.get(pos.0).unwrap().get(pos.1).unwrap()
    }

    fn get_outputs(&self, pos: (usize,usize)) -> ((usize, usize), (usize, usize)) {
        let shape = self.get_shape(pos);
        match shape {
            PipeShape::NorthSouth => ((pos.0.wrapping_sub(1), pos.1), (pos.0+1, pos.1)),
            PipeShape::NorthEast => ((pos.0.wrapping_sub(1), pos.1), (pos.0, pos.1+1)),
            PipeShape::NorthWest => ((pos.0.wrapping_sub(1), pos.1), (pos.0, pos.1.wrapping_sub(1))),
            PipeShape::WestEast => ((pos.0, pos.1.wrapping_sub(1)), (pos.0, pos.1+1)),
            PipeShape::SouthWest => ((pos.0+1, pos.1), (pos.0, pos.1.wrapping_sub(1))),
            PipeShape::SouthEast => ((pos.0+1, pos.1), (pos.0, pos.1+1)),
            PipeShape::Blank => panic!("Not a pipe, has no outputs"),
            PipeShape::StartingPosition => self.get_nearest(pos).iter().copied().collect_tuple().unwrap()
        }
    }

    fn follow_pipe(&self, entry_pos: (usize, usize), pos: (usize, usize)) -> (usize,usize) {
        let (output1, output2) = self.get_outputs(pos);

        if output1 == entry_pos {
            output2
        } else {
            output1
        }
    }

    fn get_nearest(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        // get above
        if let Some(row) = self.maze.get(pos.0.wrapping_sub(1)) {
            if let Some(shape) = row.get(pos.1) {
                match shape {
                    PipeShape::StartingPosition | PipeShape::SouthEast | PipeShape::SouthWest | PipeShape::NorthSouth => result.push((pos.0.wrapping_sub(1), pos.1)),
                    _ => {}
                }
            }
        }

        // get below
        if let Some(row) = self.maze.get(pos.0 + 1) {
            if let Some(shape) = row.get(pos.1) {
                match shape {
                    PipeShape::StartingPosition | PipeShape::NorthEast | PipeShape::NorthWest | PipeShape::NorthSouth => result.push((pos.0+1, pos.1)),
                    _ => {}
                }
            }
        }

        //get left
        if let Some(row) = self.maze.get(pos.0) {
            if let Some(shape) = row.get(pos.1.wrapping_sub(1)) {
                match shape {
                    PipeShape::StartingPosition | PipeShape::NorthEast | PipeShape::SouthEast | PipeShape::WestEast => result.push((pos.0, pos.1.wrapping_sub(1))),
                    _ => {}
                }
            }
        }

        // get right
        if let Some(row) = self.maze.get(pos.0) {
            if let Some(shape) = row.get(pos.1+1) {
                match shape {
                    PipeShape::StartingPosition | PipeShape::NorthWest | PipeShape::SouthWest | PipeShape::WestEast=> result.push((pos.0, pos.1+1)),
                    _ => {}
                }
            }
        }

        result
    }
}

impl Runner for Aoc2023_10 {
    fn name(&self) -> (usize, usize) {
        (2023, 10)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        self.maze = read_lines(&self.input)
            .iter()
            .map(|line|
                line
                    .chars()
                    .map(PipeShape::from)
                    .collect()
            ).collect();

        match self.find_starting_pos() {
            Some((row_index, col_index)) => {
                let (mut node, _) = self.get_nearest((row_index, col_index)).iter().copied().collect_tuple().unwrap();
                self.pipe.push(node);
                let mut prev_pos = (row_index, col_index);
                while node != (row_index, col_index) {
                    let temp_node = node;
                    node = self.follow_pipe(prev_pos, node);
                    self.pipe.push(node);
                    prev_pos = temp_node;
                }
            },
            _ => {
                println!("Starting position not found.");
            }
        }
    }

    fn part1(&mut self) -> i64 {
        self.pipe.len() as i64 / 2
    }

    fn part2(&mut self) -> i64 {
        0
    }
}



#[derive(PartialEq, Debug)]
enum PipeShape {
    NorthSouth, // |
    WestEast, // -
    SouthWest, // 7
    NorthWest, // J
    SouthEast, // F
    NorthEast, // L
    StartingPosition, // S,
    Blank, // .
}

impl From<char> for PipeShape {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeShape::NorthSouth,
            '-' => PipeShape::WestEast,
            '7' => PipeShape::SouthWest,
            'J' => PipeShape::NorthWest,
            'F' => PipeShape::SouthEast,
            'L' => PipeShape::NorthEast,
            'S' => PipeShape::StartingPosition,
            '.' => PipeShape::Blank,
            _ => panic!("Unknown Shape")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_10::new();

        day.set_input("crates/aoc20xx/test/2023-10.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(8, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_10::new();

        day.set_input("crates/aoc20xx/test/2023-10.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(0, result);
    }
}