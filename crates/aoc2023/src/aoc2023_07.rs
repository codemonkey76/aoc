use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};
use itertools::Itertools;

#[derive(Default)]
pub struct Aoc2023_07 {
    input: PathBuf,
    hands: Vec<Hand>,

}

impl Aoc2023_07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_07 {
    fn name(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn set_input(&mut self, input: &str) {

        println!("Setting input: {}", input);
        self.input = get_repo_root().join(input);

        println!("Setting self.input: {:?}", self.input);


    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines(&self.input);
        self.hands = lines.iter().map(Hand::from).collect();
    }

    fn part1(&mut self) -> u64 {
        let mut sorted_hands: Vec<&Hand> = self.hands.iter().clone().collect();
        sorted_hands.sort();
        sorted_hands.iter().enumerate().map(|(index, hand)| hand.bid * (index+1) as u64).sum()
    }

    fn part2(&mut self) -> u64 {
        0
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u64
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_rank().cmp(&other.get_rank()) {
            Ordering::Greater | Ordering::Less => return self.get_rank().cmp(&other.get_rank()),
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
                    let cmp_result = self_card.cmp(other_card);
                    if cmp_result != Ordering::Equal {
                        return cmp_result;
                    }
                }
            }
        }
        Ordering::Equal
    }

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Hand {
    pub fn get_rank(&self) -> u64 {
        if self.has_five_of_a_kind() {
            6
        } else if self.has_four_of_a_kind() {
            5
        } else if self.has_full_house() {
            4
        } else if self.has_three_of_a_kind() {
            3
        } else if self.has_two_pair() {
            2
        } else if self.has_pair() {
            1
        } else {
            0
        }
    }

    pub fn has_pair(&self) -> bool {
        self.cards
            .iter()
            .combinations(2)
            .any(|pair| pair[0] == pair[1])
    }

    pub fn has_two_pair(&self) -> bool {
        self.cards
            .iter()
            .combinations(2)
            .filter(|pair| pair[0] == pair[1])
            .unique()
            .count() == 2
    }

    pub fn has_three_of_a_kind(&self) -> bool {
        self.cards
            .iter()
            .combinations(3)
            .any(|combination| combination.iter().all(|&x| x == combination[0]))

    }

    pub fn has_four_of_a_kind(&self) -> bool {
        self.cards
            .iter()
            .combinations(4)
            .any(|combination| combination
                .iter()
                .all(|&x| x == combination[0])
            )

    }

    pub fn has_five_of_a_kind(&self) -> bool {
        self.cards.iter().all(|x| x == &self.cards[0])
    }

    pub fn has_full_house(&self) -> bool {
        let rank_counts = self.rank_counts();

        rank_counts.values().any(|&count| count == 3) && rank_counts.values().any(|&count| count == 2)
    }


    fn rank_counts(&self) -> HashMap<u32,usize> {
        self.cards.iter().map(|card| card.get_rank()).counts()
    }
}
impl From<&String> for Hand {
    fn from(value: &String) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        Hand {
            cards: hand.chars().map(Card::from).collect(),
            bid: bid.trim().parse().unwrap()
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Card {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
}
impl Card {
    fn get_rank (&self) -> u32 {
        match self {
            Card::Two => {2}
            Card::Three => {3}
            Card::Four => {4}
            Card::Five => {5}
            Card::Six => {6}
            Card::Seven => {7}
            Card::Eight => {8}
            Card::Nine => {9}
            Card::Ten => {10}
            Card::Jack => {11}
            Card::Queen => {12}
            Card::King => {13}
            Card::Ace => {14}
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_rank().cmp(&other.get_rank())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid Card")
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::aoc2023_07::Aoc2023_07;
    use super::*;

    #[test]
    fn part1() {
        let mut day = Aoc2023_07::new();

        day.set_input("crates/aoc2023/test/2023-07.txt");
        day.parse();
        let result = day.part1();

        assert_eq!(6440, result);
    }

    #[test]
    fn part2() {
        let mut day = Aoc2023_07::new();

        day.set_input("crates/aoc2023/test/2023-07.txt");
        day.parse();
        let result = day.part2();

        assert_eq!(0, result);
    }
}