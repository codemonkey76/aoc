use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_14 {
    input: PathBuf,
    map: Rocks
}

impl Aoc2023_14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_14 {
    fn name(&self) -> (usize, usize) {
        (2023, 14)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        self.map = aoclib::read_full(&self.input).parse().unwrap();
    }

    fn part1(&mut self) -> i64 {
        self.map.tilt(Direction::North);
        self.map.load_total()
    }

    fn part2(&mut self) -> i64 {
        let mut loop_detector = HashMap::new();

        loop_detector.insert(self.map.clone(), 0);

        let mut i = 0;
        let (start, end) = loop {
            i += 1;
            for dir in DIRS {
                self.map.tilt(dir);
            }

            if let Some(val) = loop_detector.insert(self.map.clone(), i) {
                break (val, i);
            }
        };

        let difference = end - start;
        let remaining_loops = 1000000000 - start;
        let phase = remaining_loops % difference;

        for _ in 0..phase {
            for dir in DIRS {
                self.map.tilt(dir);
            }
        }

        self.map.load_total()
    }
}

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East
];

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Rock {
    Cube,
    Round,
    Empty
}

impl Display for Rocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.map.iter().for_each(|row| {
            let s: String = row.iter().map(|rock| match rock {
                Rock::Cube => '#',
                Rock::Round => 'O',
                Rock::Empty => '.'
            }).collect();
            writeln!(f, "{s}").unwrap();
        });

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Rocks {
    map: Vec<Vec<Rock>>
}

impl Rocks {
    fn push_single(&mut self, src: (usize,usize), dest: (usize, usize)) {
        if self.map[src.0][src.1] == Rock::Round && self.map[dest.0][dest.1] == Rock::Empty {
            self.map[dest.0][dest.1] = Rock::Round;
            self.map[src.0][src.1] = Rock::Empty
        }
    }

    fn push(&mut self, x: usize, y: usize, direction: &Direction) {
        match direction {
            Direction::North => {
                for i in (1..=x).rev() {
                    self.push_single((i, y), (i-1, y));
                }
            }
            Direction::South => {
                for i in x..self.map.len()-1 {
                    self.push_single((i, y), (i+1, y));
                }
            }
            Direction::West => {
                for i in (1..=y).rev() {
                    self.push_single((x,i), (x, i-1));
                }
            }
            Direction::East => {
                for i in y..self.map[0].len()-1 {
                    self.push_single((x,i), (x, i+1));
                }
            }
        }
    }

    fn tilt(&mut self, direction: Direction) {
        let (rows, cols) = match direction {
            Direction::North => {
                ((1..self.map.len()).collect::<Vec<_>>().into_iter(),
                (0..self.map[0].len()).collect::<Vec<_>>().into_iter())
            }
            Direction::West => {
                ((0..self.map.len()).collect::<Vec<_>>().into_iter(),
                 (1..self.map[0].len()).collect::<Vec<_>>().into_iter())
            }
            Direction::South => {
                ((0..self.map.len()-1).rev().collect::<Vec<_>>().into_iter(),
                (0..self.map[0].len()).collect::<Vec<_>>().into_iter())
            }
            Direction::East => {
                ((0..self.map.len()).collect::<Vec<_>>().into_iter(),
                (0..self.map[0].len()-1).rev().collect::<Vec<_>>().into_iter())
            }
        };

        // println!("Looping rows: {rows:?}");
        // println!("Looping cols: {cols:?}");
        for i in rows {
            for j in cols.clone() {
                if self.map[i][j] == Rock::Round {
                    self.push(i, j, &direction);
                }
            }
        }
    }

    fn load_total(&self) -> i64 {
        let mut lines = self.map.len() as i64 + 1;
        self.map.iter().fold(0, |acc, line| {
            lines -= 1;
            acc + lines * line.iter().fold(0, |acc2, rock| if *rock == Rock::Round { acc2 + 1 } else { acc2 })
        })
    }
}

impl FromStr for Rocks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        s.lines().for_each(|line| {
            let row = line.chars().map(|c| match c {
                '.' => Rock::Empty,
                'O' => Rock::Round,
                '#' => Rock::Cube,
                _ => panic!("Invalid character: {c}")
            }).collect();
            map.push(row);
        });

        Ok(Rocks{ map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_14::new();

        day.set_input("crates/aoc2023/test/2023-14.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(136, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_14::new();

        day.set_input("crates/aoc2023/test/2023-14.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(64, result);
    }
}