use std::path::PathBuf;
use std::str::FromStr;
use itertools::Itertools;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_15 {
    input: PathBuf,
    steps: Vec<String>
}

impl Aoc2023_15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_15 {
    fn name(&self) -> (usize, usize) {
        (2023, 15)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input)
    }

    fn parse(&mut self) {
        self.steps = aoclib::read_full(&self.input).split(',').map(|s| s.to_string()).collect();
    }

    fn part1(&mut self) -> i64 {
        self.steps.iter().fold(0, |acc, item| acc + get_hash(item))
    }

    fn part2(&mut self) -> i64 {
        let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
        self.steps.iter().for_each(|step| {
            let instruction: Instruction = step.parse().unwrap();
            let hash = get_hash(&instruction.label) as usize;

            match instruction.instruction_type {
                InstructionType::Add => {
                    if let Some(lens) = boxes[hash].iter_mut().find(|lens| lens.label == instruction.label) {
                        if let Some(focal_length) = instruction.focal_length {
                            lens.focal_length = focal_length;
                        }
                    } else if let Some(focal_length) = instruction.focal_length {
                        boxes[hash].push(Lens { label: instruction.label.clone(), focal_length});
                    }
                },
                InstructionType::Remove => {
                    if let Some((index, _)) = boxes[hash].iter().find_position(|lens| lens.label == instruction.label) {
                        boxes[hash].remove(index);
                    }
                }
            }
        });
        let mut total: i64 = 0;

        boxes.iter().enumerate().for_each(|(i, lenses)| {
            lenses.iter().enumerate().for_each(|(j, lens)| {
                total += (i as i64+1)*(j as i64+1)*lens.focal_length as i64;
            });
        });

        total
    }
}

fn get_hash(item: &str) -> i64 {
    item.chars().fold(0, |acc, ch| {
        let mut result = acc + (ch as u8) as i64;
        result *= 17;
        result % 256
    })
}

struct Instruction {
    label: String,
    instruction_type: InstructionType,
    focal_length: Option<usize>
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        let mut label = String::new();
        let mut instruction_type = InstructionType::Add;
        let mut focal_length = None;

        while let Some(c) = iter.next() {
            match c {
                '-' => instruction_type = InstructionType::Remove,
                '=' => instruction_type = InstructionType::Add,
                '1'..='9' => {
                    let rest = iter.as_str();
                    let num_str: String = c.to_string() + rest.chars().take_while(|&x| x.is_ascii_digit()).collect::<String>().as_str();
                    focal_length = Some(num_str.parse::<usize>().unwrap());
                    break;
                }
                _ => label.push(c)
            }
        }
        Ok(Instruction { label, instruction_type, focal_length })
    }
}

enum InstructionType {
    Add,
    Remove
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_15::new();

        day.set_input("crates/aoc2023/test/2023-15.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(1320, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_15::new();

        day.set_input("crates/aoc2023/test/2023-15.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(145, result);
    }
}