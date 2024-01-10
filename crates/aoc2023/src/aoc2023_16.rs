use std::collections::HashSet;
use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_16 {
    input: PathBuf,
    grid: Vec<Vec<Tile>>
}

impl Aoc2023_16 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_16 {
    fn name(&self) -> (usize, usize) {
        (2023, 16)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        self.grid = aoclib::read_lines(&self.input)
            .iter()
            .map(|line| line.chars().map(|ch| match ch {
                '.' => Tile::Empty,
                '|' => Tile::Splitter(Splitter::Vertical),
                '-' => Tile::Splitter(Splitter::Horizontal),
                '/' => Tile::Mirror(Mirror::TopRightBottomLeft),
                '\\' => Tile::Mirror(Mirror::TopLeftBottomRight),
                _ => panic!("Invalid character")
            }).collect())
            .collect();
    }

    fn part1(&mut self) -> i64 {
        let beam = Beam { position: (0,0), direction: Direction::Right};

        follow_beams(beam, &self.grid)
    }

    fn part2(&mut self) -> i64 {

        let mut energized = Vec::new();

        for i in 0..self.grid.len() {
            energized.push(follow_beams(Beam{ position: (i, 0), direction: Direction::Right}, &self.grid));
        }

        for i in 0..self.grid.len() {
            energized.push(follow_beams(Beam{ position: (i, self.grid[0].len()-1), direction: Direction::Left}, &self.grid));
        }

        for i in 0..self.grid[0].len() {
            energized.push(follow_beams(Beam{ position: (0, i), direction: Direction::Down}, &self.grid));
        }

        for i in 0..self.grid[0].len() {
            energized.push(follow_beams(Beam{ position: (self.grid.len()-1, i), direction: Direction::Up}, &self.grid));
        }

        energized.iter().fold(0, |acc, &item| item.max(acc))
    }
}

fn follow_beams(beam: Beam, grid: &[Vec<Tile>]) -> i64 {
    let mut beams = vec![beam];
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    let mut cache: HashSet<(Direction, (usize, usize))> = HashSet::new();

    while ! beams.is_empty() {
        let mut new_beams: Vec<Beam> = Vec::new();

        for beam in beams {
            energized.insert(beam.position);
            if cache.insert((beam.direction, beam.position)) {
                match beam.follow(grid) {
                    None => {}
                    Some(FollowResult::Beam(b)) => {
                        new_beams.push(b)
                    },
                    Some(FollowResult::Beams(b1, b2)) => {
                        new_beams.push(b1);
                        new_beams.push(b2);
                    }
                }
            }
        }
        beams = std::mem::take(&mut new_beams);
    }

    energized.len() as i64
}

fn _draw_grid(grid: &[Vec<Tile>]) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|tile| match tile {
            Tile::Empty => print!("."),
            Tile::Splitter(Splitter::Vertical) => print!("|"),
            Tile::Splitter(Splitter::Horizontal) => print!("-"),
            Tile::Mirror(Mirror::TopLeftBottomRight) => print!("\\"),
            Tile::Mirror(Mirror::TopRightBottomLeft) => print!("/")
        });
        println!();
    });
}


enum FollowResult {
    Beam(Beam),
    Beams(Beam, Beam)
}

#[derive(Debug, Clone)]
struct Beam {
    position: (usize, usize),
    direction: Direction
}

impl Beam {
    fn follow_empty(&self, grid: &[Vec<Tile>]) -> Option<FollowResult> {
        let (row, col) = match self.direction {
            Direction::Up => (self.position.0 as i64 - 1, self.position.1 as i64),
            Direction::Down => (self.position.0 as i64 + 1, self.position.1 as i64),
            Direction::Left => (self.position.0 as i64, self.position.1 as i64 - 1),
            Direction::Right => (self.position.0 as i64, self.position.1 as i64 + 1)
        };

        if row < 0 || row >= grid.len() as i64 || col < 0 || col >= grid[0].len() as i64 {
            return None;
        }

        Some(FollowResult::Beam(Beam { direction: self.direction, position: (row as usize, col as usize)}))
    }

    fn split_horizontal(&self, grid: &[Vec<Tile>]) -> Option<FollowResult> {
        let left = self.position.1 as i64-1;

        if left < 0 {
            return Some(FollowResult::Beam(Beam { position: (self.position.0, self.position.1+1), direction: Direction::Right}));
        }

        let b1 = Beam { position: (self.position.0, left as usize), direction: Direction::Left};

        let right = self.position.1 + 1;

        if right >= grid[0].len() {
            return Some(FollowResult::Beam(b1));
        }

        let b2 = Beam { position: (self.position.0, right), direction: Direction::Right};

        Some(FollowResult::Beams(b1, b2))
    }

    fn split_vertical(&self, grid: &[Vec<Tile>]) -> Option<FollowResult> {
        let up = self.position.0 as i64-1;

        if up < 0 {
            return Some(FollowResult::Beam(Beam { position: (self.position.0 + 1, self.position.1), direction: Direction::Down}));
        }

        let b1 = Beam { position: (up as usize, self.position.1), direction: Direction::Up};

        let down = self.position.0 + 1;

        if down >= grid.len() {
            return Some(FollowResult::Beam(b1));
        }

        let b2 = Beam { position: (down, self.position.1), direction: Direction::Down};

        Some(FollowResult::Beams(b1, b2))
    }

    fn reflect_right(&self, grid: &[Vec<Tile>]) -> Option<FollowResult> {
        let col = self.position.1+1;
        if col >= grid[0].len() {
            return None;
        }
        Some(FollowResult::Beam(Beam { direction: Direction::Right, position: (self.position.0, col)}))
    }

    fn reflect_left(&self) -> Option<FollowResult> {
        let col = self.position.1 as i64 - 1;
        if col < 0 {
            return None;
        }
        Some(FollowResult::Beam(Beam { direction: Direction::Left, position: (self.position.0, col as usize)}))
    }

    fn reflect_up(&self) -> Option<FollowResult> {
        let row = self.position.0 as i64 - 1;
        if row < 0 {
            return None;
        }
        Some(FollowResult::Beam(Beam { direction: Direction::Up, position: (row as usize, self.position.1)}))
    }

    fn reflect_down(&self, grid: &[Vec<Tile>]) -> Option<FollowResult> {
        let row = self.position.0+1;
        if row >= grid.len() {
            return None;
        }
        Some(FollowResult::Beam(Beam { direction: Direction::Down, position: (row, self.position.1)}))
    }

    fn follow(&self, grid: &[Vec<Tile>]) -> Option<FollowResult> {
        let tile = &grid[self.position.0][self.position.1];

        match tile {
            Tile::Empty => self.follow_empty(grid),
            Tile::Splitter(split) => match split {
                Splitter::Horizontal => {
                    match self.direction {
                        Direction::Up | Direction::Down => {
                            self.split_horizontal(grid)
                        }
                        Direction::Left | Direction::Right => {
                            self.follow_empty(grid)
                        }
                    }
                }
                Splitter::Vertical => {
                    match self.direction {
                        Direction::Up | Direction::Down => {
                            self.follow_empty(grid)
                        }
                        Direction::Left | Direction::Right => {
                            self.split_vertical(grid)
                        }
                    }
                }
            }
            Tile::Mirror(mirror) => match mirror {
                Mirror::TopRightBottomLeft => {
                    match self.direction {
                        Direction::Up => self.reflect_right(grid),
                        Direction::Down => self.reflect_left(),
                        Direction::Left => self.reflect_down(grid),
                        Direction::Right => self.reflect_up()
                    }
                },
                Mirror::TopLeftBottomRight => {
                    match self.direction {
                        Direction::Up => self.reflect_left(),
                        Direction::Down => self.reflect_right(grid),
                        Direction::Left => self.reflect_up(),
                        Direction::Right => self.reflect_down(grid)
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
enum Tile {
    Empty,
    Splitter(Splitter),
    Mirror(Mirror)
}

#[derive(Debug)]
enum Splitter {
    Horizontal,
    Vertical
}

#[derive(Debug)]
enum Mirror {
    TopRightBottomLeft,
    TopLeftBottomRight
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_16::new();

        day.set_input("crates/aoc2023/test/2023-16.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(46, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_16::new();

        day.set_input("crates/aoc2023/test/2023-16.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(51, result);
    }
}