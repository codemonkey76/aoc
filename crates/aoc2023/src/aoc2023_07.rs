use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use aoclib::{get_repo_root, Runner};
use itertools::Itertools;

#[derive(Default)]
pub struct Aoc2023_07 {
    input: PathBuf,
    hands: Vec<Hand<Card>>,

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
        self.input = get_repo_root().join(input);
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines(&self.input);
        self.hands = lines.iter().map(Hand::from).collect();
    }

    fn part1(&mut self) -> u64 {
        let mut sorted_hands: Vec<&Hand<Card>> = self.hands.iter().clone().collect();
        <[&Hand<Card>]>::sort(&mut sorted_hands);
        sorted_hands.iter().enumerate().map(|(index, hand)| hand.bid * (index+1) as u64).sum()
    }

    fn part2(&mut self) -> u64 {
        let mut hands_with_jokers: Vec<Hand<CardWithJoker>>  = self.hands.iter().map(|hand| {
            let converted_cards: Vec<CardWithJoker> = hand.cards.iter().clone().map(CardWithJoker::from).collect();
            Hand {
                cards: converted_cards,
                bid: hand.bid,
            }
        }).collect();

        hands_with_jokers.sort();
        hands_with_jokers.iter().enumerate().map(|(index, hand)| hand.bid * (index+1) as u64).sum()
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Hand<T> {
    cards: Vec<T>,
    bid: u64
}

impl<T: Eq + Ord + Ranking + Hash + WildCard> Ord for Hand<T> {
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

impl<T: Eq + Ord + Ranking + Hash + WildCard> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait Ranking {
    fn get_rank(&self) -> u32;
    fn get_result(&self) -> &str;
}

trait WildCard {
    fn is_wildcard(&self) -> bool;
}

impl<T> Ranking for Hand<T> where T: Eq + Hash + Ranking + WildCard {
    fn get_rank(&self) -> u32 {
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
    fn get_result(&self) -> &str {
        match self.get_rank() {
            6 => "Five of a Kind",
            5 => "Four of a Kind",
            4 => "Full House",
            3 => "Three of a Kind",
            2 => "Two Pair",
            1 => "Pair",
            0 => "High Card",
            _ => "Unknown Card"
        }
    }
}
impl<T> Hand<T> where T: Eq + Hash + Ranking + WildCard {
    pub fn has_pair(&self) -> bool {
        self.cards
            .iter()
            .combinations(2)
            .any(|pair| pair[0] == pair[1] || pair.iter().any(|&card| card.is_wildcard()))
    }

    pub fn has_two_pair(&self) -> bool {
        self.cards
            .iter()
            .combinations(2)
            .filter(|pair| pair[0] == pair[1] || pair.iter().any(|&card| card.is_wildcard()))
            .unique()
            .count() == 2
    }

    pub fn has_three_of_a_kind(&self) -> bool {
        self.cards
            .iter()
            .combinations(3)
            .any(|combination| {
                let mut count_wildcards = 0;
                let mut reference_card = None;

                for &card in &combination {
                    if card.is_wildcard() {
                        count_wildcards += 1;
                    } else {
                        reference_card = Some(card);
                    }
                }
                count_wildcards >= 2 || combination.iter().all(|&x| x.is_wildcard() || x == reference_card.unwrap())
            })
    }

    pub fn has_four_of_a_kind(&self) -> bool {
        self.cards
            .iter()
            .combinations(4)
            .any(|combination| {
                let mut count_wildcards = 0;
                let mut reference_card = None;

                for &card in &combination {
                    if card.is_wildcard() {
                        count_wildcards += 1;
                    } else {
                        reference_card = Some(card);
                    }
                }

                count_wildcards >= 3
                    ||
                        combination.iter().all(|&x| x.is_wildcard() || x == reference_card.unwrap())
            })
    }

    pub fn has_five_of_a_kind(&self) -> bool {
        self.cards.iter().combinations(5)
            .any(|combination| {
                let mut count_wildcards = 0;
                let mut reference_card = None;

                for &card in &combination {
                    if card.is_wildcard() {
                        count_wildcards += 1;
                    } else {
                        reference_card = Some(card);
                    }
                }

                count_wildcards >= 4
                    || combination
                    .iter()
                    .all(|&x| x.is_wildcard() || x == reference_card.unwrap())
            })
    }

    pub fn has_full_house(&self) -> bool {
        let wilds = self.cards.iter().filter(|card| card.is_wildcard()).count();

        if wilds >= 3 {
            // 3 wilds will always be a full house
            true
        } else if wilds == 2 {
            // 2 wilds + a pair will always be a full house
            return self.cards.iter().filter(|card| !card.is_wildcard()).combinations(2).any(|combination| combination[0] == combination[1]);
        } else if wilds == 1 {
            self.cards
                .iter()
                .filter(|card| !card.is_wildcard())
                .combinations(2)
                .filter(|pair| pair[0] == pair[1])
                .unique()
                .count() == 2
        } else {
            let rank_counts = self.rank_counts();

            return rank_counts.values().any(|&count| count == 3) && rank_counts.values().any(|&count| count == 2);
        }
    }


    fn rank_counts(&self) -> HashMap<u32,usize> {
        self.cards.iter().map(|card| card.get_rank()).counts()
    }
}
impl<T> From<&String> for Hand<T> where T: From<char> {
    fn from(value: &String) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        Hand {
            cards: hand.chars().map(T::from).collect(),
            bid: bid.trim().parse().unwrap()
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum CardWithJoker {
    Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Queen, King, Ace
}

impl WildCard for CardWithJoker {
    fn is_wildcard(&self) -> bool {
        matches!(self, CardWithJoker::Joker)
    }
}

impl Ranking for CardWithJoker {
    fn get_rank(&self) -> u32 {
        match self {
            CardWithJoker::Joker => {1},
            CardWithJoker::Two => {2}
            CardWithJoker::Three => {3}
            CardWithJoker::Four => {4}
            CardWithJoker::Five => {5}
            CardWithJoker::Six => {6}
            CardWithJoker::Seven => {7}
            CardWithJoker::Eight => {8}
            CardWithJoker::Nine => {9}
            CardWithJoker::Ten => {10}
            CardWithJoker::Queen => {12}
            CardWithJoker::King => {13}
            CardWithJoker::Ace => {14}
        }
    }

    fn get_result(&self) -> &str {
        "Result"
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Card {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
}

impl Ranking for Card {
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

    fn get_result(&self) -> &str {
        "Result"
    }
}

impl Ord for CardWithJoker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_rank().cmp(&other.get_rank())
    }
}

impl PartialOrd for CardWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&Card> for CardWithJoker {
    fn from(value: &Card) -> Self {
        match value {
            Card::Two => CardWithJoker::Two,
            Card::Three => CardWithJoker::Three,
            Card::Four => CardWithJoker::Four,
            Card::Five => CardWithJoker::Five,
            Card::Six => CardWithJoker::Six,
            Card::Seven => CardWithJoker::Seven,
            Card::Eight => CardWithJoker::Eight,
            Card::Nine => CardWithJoker::Nine,
            Card::Ten => CardWithJoker::Ten,
            Card::Jack => CardWithJoker::Joker,
            Card::Queen => CardWithJoker::Queen,
            Card::King => CardWithJoker::King,
            Card::Ace => CardWithJoker::Ace,
        }
    }
}

impl From<char> for CardWithJoker {
    fn from(value: char) -> Self {
        match value {
            '2' => CardWithJoker::Two,
            '3' => CardWithJoker::Three,
            '4' => CardWithJoker::Four,
            '5' => CardWithJoker::Five,
            '6' => CardWithJoker::Six,
            '7' => CardWithJoker::Seven,
            '8' => CardWithJoker::Eight,
            '9' => CardWithJoker::Nine,
            'T' => CardWithJoker::Ten,
            'J' => CardWithJoker::Joker,
            'Q' => CardWithJoker::Queen,
            'K' => CardWithJoker::King,
            'A' => CardWithJoker::Ace,
            _ => panic!("Invalid Card")
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

impl WildCard for Card {
    fn is_wildcard(&self) -> bool {
        false
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

        assert_eq!(5905, result);
    }
}