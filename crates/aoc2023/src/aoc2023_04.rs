use std::collections::HashMap;
use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_04 {
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

    fn parse(&mut self) {
        let lines = aoclib::read_lines(aoclib::get_input_path(self.name()));

        for line in lines {
            let (card, data) = line.split_once(": ").unwrap();
            let (_, num) = card.split_once(" ").unwrap();
            let num = num.trim().parse::<u32>().unwrap();
            let (winners, numbers) = data.split_once(" | ").unwrap();
            self.cards.push(Card::new(num, winners, numbers));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let total = self.cards
            .iter()
            .map(|card| {
                let count = card.winning_numbers();

                if count > 0 {
                    2u32.pow(count as u32 - 1)
                } else {
                    count as u32
                }
            })
            .sum::<u32>();

        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut future_cards: HashMap<u32, u32> = HashMap::new();

        for card in &self.cards {
            let mut multiplier = 1;
            if let Some(copies) = future_cards.get(&card.id) {
                multiplier += copies;
            }
            let count = card.winning_numbers();

            for i in 0..count {
                let future_card_id = card.id+i+1;

                if let Some(card_count) = future_cards.get_mut(&future_card_id) {
                    *card_count += multiplier;
                } else {
                    future_cards.insert(future_card_id, multiplier);
                }
            }
        }
        let total = future_cards.values().sum::<u32>();
        let total = total + self.cards.len() as u32;

        aoclib::output(total)
    }
}


#[derive(Debug)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>
}

impl Card {
    fn new(id: u32, winners: &str, numbers: &str) -> Self {
        let winners = Self::parse(winners);
        let numbers = Self::parse(numbers);
        Card {
            id, winners, numbers
        }
    }
    fn parse(input: &str) -> Vec<u32> {
        input
            .split(" ")
            .filter(|num|!num.is_empty())
            .map(|num|num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    }
    fn winning_numbers(&self) -> u32 {
        self.winners
            .iter()
            .filter(|winner| self.numbers.contains(winner))
            .count() as u32
    }
}
