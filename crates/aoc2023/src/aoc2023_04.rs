use std::collections::HashMap;
use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};

#[derive(Default)]
pub struct Aoc2023_04 {
    input: PathBuf,
    cards: Vec<Card>
}

impl Aoc2023_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_04 {

    fn name(&self) -> (usize, usize) {
        (2023, 4)
    }

    fn set_input(&mut self, input: &str) {
        self.input = get_repo_root().join(input);
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines(&self.input);

        for line in lines {
            let (card, data) = line.split_once(": ").unwrap();
            let (_, num) = card.split_once(' ').unwrap();
            let num = num.trim().parse::<i32>().unwrap();
            let (winners, numbers) = data.split_once(" | ").unwrap();
            self.cards.push(Card::new(num, winners, numbers));
        }
    }

    fn part1(&mut self) -> i64 {
        let total: i64 = self.cards
            .iter()
            .map(|card| {
                let count = card.winning_numbers();

                if count > 0 {
                    2i64.pow(count as u32 - 1)
                } else {
                    count as i64
                }
            })
            .sum();

        total
    }

    fn part2(&mut self) -> i64 {
        let mut future_cards: HashMap<i32, i32> = HashMap::new();

        self.cards
            .iter()
            .for_each(|card| {
            let multiplier = *future_cards.get(&card.id).unwrap_or(&0) + 1;

            let count = card.winning_numbers();

            (1..=count).for_each(|i| {
                let future_card_id = card.id+i;

                future_cards
                    .entry(future_card_id)
                    .and_modify(|card_count| *card_count += multiplier)
                    .or_insert_with(|| multiplier);
            });
        });

        let total: i64 = future_cards.values().map(|num| *num as i64).sum();

        total + self.cards.len() as i64
    }
}


#[derive(Debug)]
struct Card {
    id: i32,
    winners: Vec<i32>,
    numbers: Vec<i32>
}

impl Card {
    fn new(id: i32, winners: &str, numbers: &str) -> Self {
        let winners = Self::parse(winners);
        let numbers = Self::parse(numbers);
        Card {
            id, winners, numbers
        }
    }
    fn parse(input: &str) -> Vec<i32> {
        input
            .split_whitespace()
            .filter(|num|!num.is_empty())
            .map(|num|num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }
    fn winning_numbers(&self) -> i32 {
        self.winners
            .iter()
            .filter(|winner| self.numbers.contains(winner))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2023_04::Aoc2023_04;
    use super::*;

    #[test]
    fn part1() {
        let mut day4 = Aoc2023_04::new();

        day4.set_input("crates/aoc2023/test/2023-04.txt");
        day4.parse();
        let result = day4.part1();

        assert_eq!(13, result);


    }

    #[test]
    fn part2() {
        let mut day4 = Aoc2023_04::new();

        day4.set_input("crates/aoc2023/test/2023-04.txt");
        day4.parse();
        let result = day4.part2();
        assert_eq!(30, result);
    }
}