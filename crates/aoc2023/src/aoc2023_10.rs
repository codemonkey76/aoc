use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use itertools::Itertools;
use aoclib::{get_repo_root, read_lines, Runner};

#[derive(Default)]
pub struct Aoc2023_10 {
    input: PathBuf,
    maze: Vec<Vec<PipeShape>>,
    pipe: Vec<(usize, usize)>,
    visited: HashSet<(usize, usize)>,
    inside_loop: HashSet<(usize, usize)>
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


    fn _draw_maze(&self) {
        self.maze.iter().enumerate().for_each(|(row_index, row) | {
            row.iter().enumerate().for_each(|(col_index, shape)| {
                if self.visited.contains(&(row_index, col_index)) {
                    print!("{}", shape);
                } else if self.inside_loop.contains(&(row_index, col_index)) {
                    print!("I");
                    // print!("({},{})", row_index, col_index);
                } else {
                    print!(".");
                }
            });
            println!();
        })
    }

    fn get_starting_shape(&self) -> PipeShape {
        if let Some(start_position) = self.find_starting_pos() {
            // Get surrounding shapes:
            let north = self
                .maze
                .get(start_position.0.wrapping_sub(1))
                .and_then(|row| row.get(start_position.1))
                .unwrap_or(&PipeShape::Blank);

            let west = self
                .maze
                .get(start_position.0)
                .and_then(|row| row.get(start_position.1.wrapping_sub(1)))
                .unwrap_or(&PipeShape::Blank);

            let east = self
                .maze
                .get(start_position.0)
                .and_then(|row| row.get(start_position.1+1))
                .unwrap_or(&PipeShape::Blank);

            let south = self
                .maze
                .get(start_position.0+1)
                .and_then(|row| row.get(start_position.1))
                .unwrap_or(&PipeShape::Blank);

            match (north.has_output(Direction::South), west.has_output(Direction::East), east.has_output(Direction::West), south.has_output(Direction::North)) {
                (true, true, false, false) => PipeShape::NorthWest,
                (true, false, true, false) => PipeShape::NorthEast,
                (true, false, false, true) => PipeShape::NorthSouth,
                (false, true, true, false) => PipeShape::WestEast,
                (false, true, false, true) => PipeShape::SouthWest,
                (false, false, true, true) => PipeShape::SouthEast,
                _ => panic!("Too many blocks point into the start block")
            }
        } else {
            PipeShape::Blank
        }
    }

    fn compute_inside(&mut self) {
        // println!("Computing what tiles are inside");
        let mut inside = false;
        let line_entered_from = Direction::North;

        let mut inside_loop = HashSet::new();

        // println!("Starting outside");
        self.maze.iter().enumerate().for_each(|(row_index, row) | {
            row.iter().enumerate().for_each(|(col_index, shape)| {
                if self.visited.contains(&(row_index, col_index)) {
                    let mut shape = *shape;
                    if shape == PipeShape::StartingPosition {
                        shape = self.get_starting_shape();
                        println!("Starting Shape: {:?}", shape);
                    }
                    match shape {
                        PipeShape::NorthSouth => inside = !inside,
                        PipeShape::WestEast => {}
                        PipeShape::SouthWest => {
                            if line_entered_from == Direction::North {
                                inside = !inside
                            }
                        },
                        PipeShape::NorthWest => {
                            if line_entered_from == Direction::South {
                                inside = !inside
                            }
                        }
                        PipeShape::SouthEast => {
                            if line_entered_from == Direction::North {
                                inside = !inside
                            }
                        }
                        PipeShape::NorthEast => {
                            if line_entered_from == Direction::South {
                                inside = !inside
                            }
                        }
                        PipeShape::StartingPosition => {
                        }
                        PipeShape::Blank => {}
                    }
                }
                if inside {
                    inside_loop.insert((row_index, col_index));
                }

            });
            if inside {
                println!("Error on row: {}", row_index);
            }
            inside = false;
        });

        self.inside_loop = inside_loop;
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

    fn follow_pipe(&mut self, entry_pos: (usize, usize), pos: (usize, usize)) -> (usize,usize) {
        self.visited.insert(pos);
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
                //Rewrite the starting PipShape to be the correct type
                *self.maze.get_mut(row_index).unwrap().get_mut(col_index).unwrap() = self.get_starting_shape();

                // Add the starting position as the first entry in the pipe vector
                let start_pos = (row_index, col_index);
                self.pipe.push(start_pos);
                self.visited.insert(start_pos);

                // Get the 2 nodes that lead into the starting node
                let (mut node1, _) = self.get_nearest(start_pos).iter().copied().collect_tuple().unwrap();
                let mut prev_pos = start_pos;
                self.pipe.push(node1);

                while node1 != start_pos {
                    let temp_node = node1;
                    node1 = self.follow_pipe(prev_pos, node1);
                    if node1 != start_pos {
                        self.pipe.push(node1);
                    }
                    prev_pos = temp_node;
                }

            },
            _ => {
                println!("Starting position not found.");
            }
        }
        self.compute_inside();
        // self.draw_maze();
    }

    fn part1(&mut self) -> i64 {
        self.pipe.len() as i64 / 2
    }

    fn part2(&mut self) -> i64 {
        let results: HashSet<(usize, usize)> = self.inside_loop.difference(&self.visited).cloned().collect();
        results.len() as i64
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(PartialEq, Debug, Clone, Copy)]
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

impl PipeShape {
    fn has_output(&self, direction: Direction) -> bool {
        matches!((self, direction),
            (PipeShape::NorthSouth, Direction::North | Direction::South) |
            (PipeShape::NorthWest, Direction::North | Direction::West) |
            (PipeShape::NorthEast, Direction::North | Direction::East) |
            (PipeShape::WestEast, Direction::West  | Direction::East) |
            (PipeShape::SouthWest, Direction::South | Direction::West) |
            (PipeShape::SouthEast, Direction::South | Direction::East))
    }
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

impl Display for PipeShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PipeShape::StartingPosition => write!(f, "S"),
            PipeShape::NorthSouth => write!(f, "│"),
            PipeShape::WestEast => write!(f, "─"),
            PipeShape::SouthWest => write!(f, "┐"),
            PipeShape::NorthWest => write!(f, "┘"),
            PipeShape::SouthEast => write!(f, "┌"),
            PipeShape::NorthEast => write!(f, "└"),
            PipeShape::Blank => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_10::new();

        day.set_input("crates/aoc2023/test/2023-10.txt");
        day.parse();
        println!("Pipe: {:?}", day.pipe);
        let result = day.part1();

        assert_eq!(8, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_10::new();

        day.set_input("crates/aoc2023/test/2023-10b.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(10, result);
    }
}