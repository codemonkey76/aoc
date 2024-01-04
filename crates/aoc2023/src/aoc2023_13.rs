use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use aoclib::{get_repo_root, Runner, transpose};

#[derive(Default)]
pub struct Aoc2023_13 {
    input: PathBuf,
    patterns: Vec<Pattern>
}

impl Aoc2023_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_13 {
    fn name(&self) -> (usize, usize) {
        (2023, 13)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        aoclib::read_groups(&self.input)
            .iter()
            .for_each(|group| self
                .patterns
                .push(group.parse().unwrap()));
    }

    fn part1(&mut self) -> i64 {
        self.patterns.iter().map(|pattern| pattern.get_score()).sum::<u64>() as i64
    }

    fn part2(&mut self) -> i64 {
        self.patterns.iter().map(|pattern| pattern.get_score2()).sum::<u64>() as i64
    }
}

#[derive(Debug, PartialEq)]
enum Axis {
    Horizontal,
    Vertical
}

#[derive(Debug)]
struct Reflection {
    axis: Axis,
    position: usize
}

impl Reflection {
    fn score(&self) -> u64 {
        match self.axis {
            Axis::Vertical => self.position as u64,
            Axis::Horizontal => self.position as u64 * 100
        }
    }
}
#[derive(Debug)]
struct Pattern {
    map: Vec<Vec<bool>>
}

impl Pattern {
    fn get_score(&self) -> u64 {
        if let Some(reflection) = self.get_reflection() {
            reflection.score()
        } else {
            println!("Could not find reflection for pattern");
            println!("{}", self);
            0
        }
    }

    fn get_score2(&self) -> u64 {
        if let Some(reflection) = self.get_reflection() {
            let score = reflection.score();
            let map = self.map.clone();

            for i in 0..map.len() {
                for j in 0..map[0].len() {
                    let mut new_pattern = Pattern { map: map.clone() };
                    new_pattern.map[i][j] = !map[i][j];

                    if let Some(new_reflection) = new_pattern.get_new_reflection(&reflection) {
                        let new_score = new_reflection.score();
                        if  new_score != score {
                            return new_score;
                        }
                    }
                }
            }
            panic!("No new line of reflection found for pattern");
        } else {
            println!("Could not find reflection for pattern");
            println!("{}", self);
            0
        }
    }

    fn get_reflection_pos(map: &[Vec<bool>]) -> Option<usize> {
        Pattern::get_reflection_pos_with_ignore(map, None)
    }

    fn get_reflection_pos_with_ignore(map: &[Vec<bool>], ignore: Option<usize>) -> Option<usize> {
        let mut result = None;

        for i in 1..map.len() {
            if Some(i) == ignore { continue; }
            let comparison_window = i.min(map.len()-i);

            if map[i-comparison_window..i].iter().rev().cloned().collect::<Vec<Vec<bool>>>() == map[i..i+comparison_window].to_vec() {
                result = Some(i);
                break;
            }
        }

        result
    }

    fn get_new_reflection(&self, old_reflection: &Reflection) -> Option<Reflection> {
        match old_reflection.axis {
            Axis::Horizontal => {
                if let Some(pos) = Self::get_reflection_pos_with_ignore(&self.map, Some(old_reflection.position)) {
                    return Some(Reflection { axis: Axis::Horizontal, position: pos });
                }

                let map = transpose(&self.map);

                if let Some(pos) = Self::get_reflection_pos(&map) {
                    return Some(Reflection{ axis: Axis::Vertical, position: pos });
                }
            }
            Axis::Vertical => {
                if let Some(pos) = Self::get_reflection_pos(&self.map) {
                    return Some(Reflection { axis: Axis::Horizontal, position: pos });
                }

                let map = transpose(&self.map);

                if let Some(pos) = Self::get_reflection_pos_with_ignore(&map, Some(old_reflection.position)) {
                    return Some(Reflection{ axis: Axis::Vertical, position: pos });
                }
            }
        }

        None
    }

    fn get_reflection(&self) -> Option<Reflection> {
        if let Some(pos) = Self::get_reflection_pos(&self.map) {
            return Some(Reflection { axis: Axis::Horizontal, position: pos });
        }

        let map = transpose(&self.map);

        if let Some(pos) = Self::get_reflection_pos(&map) {
            return Some(Reflection{ axis: Axis::Vertical, position: pos });
        }

        None
    }

}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.map.iter().for_each(|row| {
            writeln!(f, "{}", row.iter().map(|val| {
                if *val { "#" } else { "." }
            }).collect::<String>()).unwrap();
        });
        Ok(())
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();

        s.lines().for_each(|line| map.push(
            line.chars().map(|c| c=='#').collect()
        ));

        Ok(Pattern { map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_13::new();

        day.set_input("crates/aoc2023/test/2023-13.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(405, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_13::new();

        day.set_input("crates/aoc2023/test/2023-13.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(400, result);
    }
}